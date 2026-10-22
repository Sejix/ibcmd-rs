# XDTO-модель форм: ID = позиция свойства

Дата: 22.10.2026. Ветка: `feat/rea-form-property-inventory-20261022`.

## Источник истины

Пакет `http://v8.1c.ru/8.3/xcf/logform` лежит **открытым XML** внутри
`mngbase_root.res` платформы. Это не догадка и не EDT: та же модель, которую
использует конфигуратор.

Модель — 66 типов в 8.3.x и 68 в 8.5.1; у каждого типа есть `base` и список
`property` в порядке объявления.

## Правило ID

ID атрибута в писателе XML — **0-based позиция свойства в развёрнутой цепочке
наследования**, начиная с корня:

```
PredefinedChildItems (9)
  → FormVisualEntity (2)
    → FormItemBase (id, name, DisplayImportance)
      → GroupBase (…)
        → UsualGroup (30)
```

Проверка на живых данных:

| ID | Свойство | Владелец | Умолчание |
|---|---|---|---|
| `0xb` | `id` | `FormItemBase` | — |
| `0xc` | `name` | `FormItemBase` | — |
| `0xd` | `DisplayImportance` | `FormItemBase` | — |
| `0x1f` | `Group` | `UsualGroup` | `Vertical` |
| `0x20` | `ChildrenAlign` | `UsualGroup` | `Auto` |
| `0x25` | `Behavior` | `UsualGroup` | `Auto` |

Это ровно те значения, которые пишет нативный писатель: у `UsualGroup`
последовательность начинается `0x1f, 0x20, 0x21…`, у `Table` — `0xc, 0xb, 0xd,
0xe…`.

## Проверка

`tools/rea/xdto_model.py` извлекает модель из `.res`, а команда `verify`
сверяет с ней нативные последовательности.

Для 8.5.1.1529 все ID всех десяти элементов нашлись в модели: `UsualGroup` 30,
`Table` 89, `Button` 49, `Field` 55, `Form` 38, `Pages` 3, `Page` 16,
`CommandBar` 3, `ContextMenu` 1, `ColumnGroup` 12.

Модели выгружены для шести платформ: 8.3.21.1895, 8.3.24.1819, 8.3.27.1989,
8.3.27.2214, 8.5.1.1150, 8.5.1.1529.

## Почему карта из EDT была неверна

EDT-плагин `form.export.xml` задаёт **свой** порядок свойств, а не порядок
XDTO. Сверка с моделью:

| Элемент | Расхождений |
|---|---|
| `UsualGroup` | 13 из 30 |
| `Table` | 89 из 89 |

Поэтому карты `id-name-*-UsualGroup.json` и `id-name-8.5.1-Table.json` в
`docs/evidence/rea-form-writers-20261022/` считаются устаревшими: для имён
используется только XDTO.

## Именованные изменения 8.3.27.2214 → 8.5.1.1529

| Тип | Добавлено | Удалено | Изменены умолчания |
|---|---|---|---|
| UsualGroup | CollapsedRepresentationItem, ChildItemsTitleLocation, ShowAsCard, BackPictureEffect, CardRepresentationType, Hyperlink, BackPicture, ScrollOnCompress | — | ShowTitle |
| Table | HorizontalLinesBWA, VerticalLinesBWA, UseAlternationRowColorBWA, MarkRequiredComplete, InitialRowActivation, RowActionsShowType, HierarchyPanelLocation, SaveColors, IconSaveColorsKey, ShapeSaveColorsKey, ShowCommandBar, MaxCardHeight, CardBehaviorOnVerticalCompression, CellHyperlinksRepresentation | HorizontalLines, VerticalLines, UseAlternationRowColor | — |
| Button | ShowAsCard, BackPicture, BackPictureEffect, CardPictureAndTitleAlign, PictureHeight, ButtonImportance | — | — |
| Field | ShowInCard, FixInCard, MarkRequiredComplete, CellMark, AutoEditMode, AppearanceInCard, ShowTitleInCard, WidthInCard, AutoWidthInTable, CellHyperlinkRepresentation, CellHyperlinkDisplayVariant | — | — |
| Form | Title, ChildItemsTitleLocation, ScaleVariant, CreateButtonsGroupTitle, CreateButtonsGroupPicture, WindowViewMode, ShowCommandBar | — | WindowOpeningMode, ShowTitle |
| Page | ChildItemsTitleLocation | — | ShowTitle, ScrollOnCompress |
| CommandBar | AppearanceMode | — | — |

Отдельно: в 8.5 писатели `Button` и `Table` стали писать `id` и `name` тем же
циклом, что и остальные свойства (в 8.3 их писал базовый код).

## Воспроизведение

```powershell
python tools/rea/xdto_model.py extract `
  --res "C:/Program Files/1cv8/8.5.1.1529/bin/mngbase_root.res" `
  --out docs/evidence/rea-xdto-20261022/8.5.1.1529.json

python tools/rea/xdto_model.py verify `
  --xdto docs/evidence/rea-xdto-20261022/8.5.1.1529.json `
  --native docs/evidence/rea-form-writers-20261022/8.5.1.1529.json

python tools/rea/xdto_model.py map `
  --xdto docs/evidence/rea-xdto-20261022/8.5.1.1529.json `
  --native docs/evidence/rea-form-writers-20261022/8.5.1.1529.json `
  --out docs/evidence/rea-xdto-20261022/8.5.1.1529-mapped.json
```
