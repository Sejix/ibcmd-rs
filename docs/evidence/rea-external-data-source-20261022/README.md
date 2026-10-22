# Внешний источник: строки хранения и целевой XML

Дата: 22.10.2026. Источник фактов: строки `Config` базы `ibcmd_map`
(ServiceDesk 3.10.28, платформа 8.5.1.1529), распакованные raw-deflate, и
нативная выгрузка того же объекта.

## Коды объектов

| Код записи | Что это | Файл |
|---|---|---|
| `2` | `ExternalDataSource` | `ExternalDataSources/<Имя>.xml` |
| `11` | `ExternalDataSourceTable` | `ExternalDataSources/<Имя>/Tables/<Таблица>.xml` |

Код читается из второго элемента записи: `{1,{<код>,…}}`.

## Строка внешнего источника (код 2)

```
{1,{2,
  {0,{3,{1,0,<uuid>},"ВнешнийИсточникДанных1",{0},"",0,0,nil,0}},
  faa8f956-…,035a443a-…,      // Manager:          TypeId, ValueId
  fb73f918-…,d4f69b50-…,      // TablesManager:    TypeId, ValueId
  2d555b64-…,309c1150-…,      // CubesManager:     TypeId, ValueId
  0,0},3,
  {2bb208ca-e441-4023-9ab9-32a7807a85d0,0},   // коллекция, пустая
  {a98a108a-49a0-4dcf-bfd0-aa4b89eeb3f5,0},   // коллекция, пустая
  {e3403acd-1c95-421b-87e4-4dfa29d38b52,1,<uuid таблицы>}  // Tables
}
```

`e3403acd-…` — id коллекции таблиц, он же используется в `ConfigDumpInfo`
(см. `docs/evidence/rea-export-parity-20261022/README.md`). Элементы
коллекции — `{<id коллекции>,<count>,<uuid>,…}`.

## Строка таблицы (код 11)

```
{1,{11,
  {0,{3,{1,0,<uuid>},"oc_product_to_1c",{0},"",0,0,nil,0}},
  59d4ebec-…,5c8729a1-…,  // Manager
  f668425d-…,fa2dded7-…,  // RecordSet
  f4e6c949-…,804897a4-…,  // Record
  f1f0a422-…,152b87c0-…,  // RecordKey
  5451dc44-…,fb34fae5-…,  // Object
  dd86ab6c-…,39a77753-…,  // Ref
  516abe02-…,7f339566-…,  // List
  0,"oc_product_to_1c",
  {0,1,{"#",157fa490-4ce9-11d4-9415-008048da11f9,{1,<uuid поля product_id>}}},
  … nil-ссылки, свойства таблицы …
  57f5e963-…,537441e5-…,  // RecordManager
  {"U"},0,{1,2,0},0,0,{0,"","",{0,0,0}},0,0,nil,1,{0,0},{1,{0,0}},0},4,
  {17816ebc-…,0},{3daea016-…,0},{5bb6f09e-…,0},   // пустые коллекции
  {c7075688-…,2,                                  // Fields: 2 поля
    {{1,{27,{2,{3,{1,0,<uuid>},"product_id",{0},"",0,0,nil,0},
      {"Pattern",{"N",10,0,0}}},
      … флаги и умолчания …,{"N",0},0,0,0},"product_id",0,0},0},
    {{1,{27,{2,{3,{1,0,<uuid>},"_c_id",{0},"",0,0,nil,0},
      {"Pattern",{"S",255,1}}},
      … ,{"S",""},0,0,0},"1c_id",0,0},0}
  }
}
```

Порядок семи пар сгенерированных типов в строке **не совпадает** с порядком
`<xr:GeneratedType>` в XML. XML пишет их в порядке:
`Manager, Object, Ref, List, Record, RecordSet, RecordKey, RecordManager`
(восемь типов; в строке пар семь — `RecordManager` стоит отдельно, в области
свойств, перед `4`).

## Что пишет платформа

`ExternalDataSources/<Имя>.xml`:

- `<InternalInfo>`: три `GeneratedType` — `ExternalDataSourceManager.X`
  (Manager), `ExternalDataSourceTablesManager.X` (TablesManager),
  `ExternalDataSourceCubesManager.X` (CubesManager), каждая с `TypeId`/`ValueId`;
- `<Properties>`: `Name`, `Synonym`, `Comment`, `DataLockControlMode`;
- `<ChildObjects><Table>Имя</Table>…</ChildObjects>`.

`ExternalDataSources/<Имя>/Tables/<Таблица>.xml`:

- `<InternalInfo>`: восемь `GeneratedType` (`Manager`, `Object`, `Ref`,
  `List`, `Record`, `RecordSet`, `RecordKey`, `RecordManager`) с именами
  `ExternalDataSourceTable<Category>.<Источник>.<Таблица>`;
- `<Properties>`: `Name`, `Synonym`, `Comment`, `TableType`
  (`Table`/`View`?), `NameInDataSource`, `ExpressionInDataSource`,
  `TableDataType` (`ObjectData`/`NonobjectData`), `KeyFields` (ссылки на поля),
  `PresentationField`, `ParentField`, `UnfilledParentValue`,
  `Characteristics`, `UseStandardCommands`, `QuickChoice`, `InputByString`,
  `CreateOnInput`, `SearchStringModeOnInputByString`,
  `ChoiceDataGetModeOnInputByString`, … (полный список — в нативном файле);
- `<ChildObjects><Field>…</Field>…</ChildObjects>`; у поля — `Name`,
  `Synonym`, `Comment`, `Type` (тип с квалификаторами), `PasswordMode`,
  `Format`, `EditFormat`, `ToolTip`, `MarkNegatives`, `Mask`, `MultiLine`,
  `ExtendedEdit`, `MinValue`, `MaxValue`, `FillFromFillingValue`, `FillValue`,
  `FillChecking`, `ChoiceParameterLinks`, `ChoiceParameters`, `QuickChoice`,
  `CreateOnInput`, `ChoiceHistoryOnInput`, `ChoiceForm`,
  `NameInDataSource`, `ReadOnly`, `AllowNull`.

## Типы полей

Тип поля хранится в записи поля как `{"Pattern", <тип>}`:

| Запись | XML |
|---|---|
| `{"N",10,0}` | `<v8:Type>xs:decimal</v8:Type>` + `Digits=10`, `FractionDigits=0` |
| `{"S",255,1}` | `<v8:Type>xs:string</v8:Type>` + `Length=255`, `AllowedLength=Variable` |

Остальные коды (`{"B"}`, даты, ссылки) нужно снять с платформы так же, как
цвета: не по одному примеру, а полной таблицей.

## План реализации

1. Признать код `11` видом `ExternalDataSourceTable` (сейчас строка попадает
   в `<unknown>`), зарегистрировать её имя в `NameIndex`
   (`ExternalDataSource.<Источник>.Table.<Таблица>`).
2. Разобрать строку кода `2`: заголовок, три пары типов, коллекцию таблиц.
   Писать `ExternalDataSources/<Имя>.xml`, включая `<ChildObjects>`.
3. Разобрать строку кода `11`: восемь типов, свойства, коллекцию полей.
   Писать `<Источник>/Tables/<Таблица>.xml`.
4. Таблицу типов полей построить по полному набору кодов из платформы.
5. Проверка: оба файла побайтово совпадают с нативными; fallback-строк — 0.

## Открытые вопросы

- Полный список `TableType`/`TableDataType` и умолчаний — снять с платформы.
- Полный набор кодов типов полей — из ресурсов/декомпиляции.
- Кубы (`ExternalDataSourceCube`) — отдельный вид, пока не встречался.
