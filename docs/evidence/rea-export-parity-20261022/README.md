# Побайтовое сравнение выгрузок: native ibcmd против ibcmd-rs

Дата: 22.10.2026. Ветка: `feat/rea-form-property-inventory-20261022`.

## Стенд

- Платформа: 1С 8.5.1.1529.
- Конфигурация: ServiceDesk 3.10.28 (файл `ServiceDesk-3.10.28-release.cf`, 18 МБ).
- База: MS SQL Server, создана нативным `ibcmd infobase create --load`, применена
  `ibcmd config apply --force`.
- Выгрузки: нативная в `D:\temp\native-export`, `ibcmd-rs` в `tmp/rs-export`
  (сборка `cargo build`, debug, коммит ветки).

Команды (обе выгрузки из одной базы, один и тот же `--platform 8.5.1.1529`):

```powershell
ibcmd infobase config export --dbms=MSSQLServer --db-server=localhost `
  --db-name=ibcmd_map --db-user=ibcmd_probe --db-pwd=... D:\temp\native-export

ibcmd-rs infobase config export --dbms=MSSQLServer --db-server=localhost `
  --db-name=ibcmd_map --db-user=ibcmd_probe --db-pwd=... `
  --platform 8.5.1.1529 tmp/rs-export
```

## Результат

| Показатель | Значение |
|---|---|
| Файлов у нативного | 1 961 |
| Файлов у `ibcmd-rs` | 1 958 |
| Общих | 1 958 |
| Совпали побайтово (SHA-256) | **1 949** |
| Различаются | **9** |
| Только у нативного | 3 (включая `ConfigDumpInfo.xml`) |

`ibcmd-rs` завершился с кодом −1:

```
ConfigDumpInfo has 1 entries without canonical routes
[78afc3f6-2d63-4482-a33e-fd11f668c2b6: ... has no canonical metadata reference]
```

То есть два файла внешних источников и сам `ConfigDumpInfo.xml` не записались
из-за этой ошибки, а не из-за расхождения содержимого.

## Различающиеся файлы

Числа — строки, которых нет в одном из деревьев (`Compare-Object`).

| Файл | Только у нативного | Только у ibcmd-rs | Первое различие |
|---|---|---|---|
| `DataProcessors\ТаймТрекер\Forms\Форма\Ext\Form.xml` | 11 | 0 | `<DimensionItemHyperlink>true</DimensionItemHyperlink>` |
| `Reports\Gant1C\Templates\пмГант_Гант\Ext\Template.xml` | 368 | 1 | `<leftBorder>1</leftBorder>` |
| `DataProcessors\РабочийСтолОбращенийТП\Templates\Обращение\Ext\Template.xml` | 2 | 1 | `<textColor>style:FieldTextColor</textColor>` |
| `DataProcessors\ТелеграмОбновлениеВерсии.xml` | 4 | 4 | `<xr:Ref>0:3b2a508b-…</xr:Ref>` |
| `Subsystems\Проекты.xml` | 1 | 1 | ссылка на таблицу внешнего источника: у нативного UUID, у нас `ExternalDataSource.…Table.…` |
| `Catalogs\узСпринты\Forms\ФормаЭлемента\Ext\Form.xml` | 1 | 0 | `<MainTable>1:33ee2945-…</MainTable>` |
| `DataProcessors\БотТелеграм\Forms\Форма\Ext\Form.xml` | 1 | 0 | `<TitleDataPath>1/0:dff8c7e9-…/100000000</TitleDataPath>` |
| `Documents\ОбращениеВПоддержку\Forms\ФормаДокумента\Ext\Form.xml` | 1 | 0 | `<Output>Enable</Output>` |
| `Ext\ClientApplicationInterface.xml` | 1 | 0 | `<height>1</height>` |

## Что это значит

- Базовая выгрузка уже почти совпадает: **99,2% файлов** идентичны побайтово.
- Основной класс расхождений — **пропущенные свойства** (10 файлов из 12):
  `TextColor`, `MainTable`, `TitleDataPath`, `Output`, `DimensionItemHyperlink`,
  `height`, элементы оформления шаблона. Это не порядок и не имена — значения
  просто не пишутся.
- Отдельный класс — **маршрутизация ссылок** на внешние источники данных:
  `ConfigDumpInfo` не находит канонический маршрут, из-за чего падает вся
  выгрузка и теряются три файла.
- Расхождения порядка в этом наборе не проявились.

## Блокер ConfigDumpInfo: разбор строки

Строка `Config` с именем `78afc3f6-…` — это таблица внешнего источника. Её
содержимое (inflate, raw deflate):

```
{1,{11,{0,{3,{1,0,78afc3f6-…},"oc_product_to_1c",{0},"",0,0,00000000-…,0}},…}}
```

Владельца в строке нет. Связь лежит в строке самого внешнего источника
(`9565660f-…`):

```
{1,{2,{0,{3,{1,0,9565660f-…},"ВнешнийИсточникДанных1",…}},…,
 {e3403acd-…,1,78afc3f6-…}}
```

То есть таблица — элемент коллекции `{<id коллекции>,1,<uuid таблицы>}`, а
`ConfigDumpInfo` ждёт имя
`ExternalDataSource.ВнешнийИсточникДанных1.Table.oc_product_to_1c`.
Канонические имена строятся из `object_refs`, куда вложенные объекты не
попадают, поэтому строка остаётся безымянной и выгрузка падает.

План: индексировать все строки метаданных по uuid, для вложенных объектов
находить родителя по такой коллекции и собирать имя
`<родитель>.<Table|Field>.<имя>`; поля таблицы лежат в её строке.

## Следующие шаги

1. Починить маршрут `ExternalDataSource.…Table.…` в `ConfigDumpInfo`, чтобы
   выгрузка завершалась и писала все файлы.
2. Разобрать пропуски свойств: `TextColor` (web-цвета), `MainTable`,
   `TitleDataPath`, `Output`, `DimensionItemHyperlink`.
3. Шаблон Ганта (`Template.xml`, 368 строк) — отдельная задача, вероятно
   палитра/оформление.
4. После исправлений повторить сравнение: цель — 1 961 из 1 961.
