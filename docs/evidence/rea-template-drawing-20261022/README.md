# Шаблон Ганта: `drawing` с объектом `GanttChart`

Дата: 22.10.2026. Последний файл, мешающий полному паритету:
`Reports/Gant1C/Templates/пмГант_Гант/Ext/Template.xml` (1960 из 1961
совпадают).

## Что расходится

В нативном шаблоне есть элемент `<drawing>` с вложенным объектом
`d3p1:GanttChart`; в нашей выгрузке его нет вовсе. Плюс связанные с ним
`formatIndex` и границы ячеек — всего 369 строк различий.

## Структура `<drawing>`

Дочерние элементы в порядке платформы:

```
drawingType          GanttChart
id                   1
formatIndex          61
text                 <v8:item><v8:lang>ru</v8:lang><v8:content>Начало</v8:content></v8:item>
beginRow / beginRowOffset
endRow / endRowOffset
beginColumn / beginColumnOffset
endColumn / endColumnOffset
autoSize             false
pictureSize          Stretch
zOrder               1
object               xsi:type="d3p1:GanttChart"
```

Объект `d3p1:GanttChart` несёт `<d3p1:chart>` с десятками настроек:
`seriesCurId`, `pointsCurId`, `isSeriesDesign`, `realSeriesCount`,
`realExSeriesData` (цвет, линия, маркер), `isPointsDesign`,
`realPointCount`, `curSeries`, `curPoint`, `chartType`, `circleLabelType`,
`labelsDelimiter`, `labelsLocation`, `lbFormat`, `lbpFormat`, `labelsColor`,
`labelsFont` (`AutoFont`), `transparentLabelsBkg`, `labelsBkgColor`,
`labelsBorder` и далее.

## Где это хранится

Пока не локализовано: `moxel.rs` (писатель шаблонов) вообще не упоминает
`drawing`, а запись MXL разбирается на форматы и палитру. Нужен дамп полей
записи шаблона и поиск структуры, содержащей `GanttChart`.

## Что уже есть и может пригодиться

- Для форм уже есть сериализатор настроек диаграммы Ганта
  (`mssql_dump/form_body.rs`, namespace `d3p1`), его таблицы свойств можно
  переиспользовать, если запись шаблона хранит те же настройки.
- `parse_moxel_*` уже умеет форматы и палитру; `drawing` — отдельная ветка.

## План

1. Дампом полей записи шаблона найти слот, где лежит `GanttChart`, и снять его
   структуру (как делали для `PlannerField`).
2. Сопоставить настройки с уже известным сериализатором форм; недостающие
   снять с платформы.
3. Написать `<drawing>` (порядок элементов выше) и проверить побайтово.
4. После совпадения — 1961 из 1961.

## Почему это отдельная задача

Это не свойство и не цвет, а целый объект диаграммы с собственной моделью.
Остальные 1960 файлов уже совпадают побайтово.
