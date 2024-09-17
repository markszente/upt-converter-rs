# UPT Converter

## What

UPT is a proprietary format devised by Neptun (a Hungarian [LMS](https://en.wikipedia.org/wiki/Learning_management_system)) to export and import questions and answers in their Unipol online quiz system.

## Why

In 2021, business requirement arose that a tool to convert UPT export files to a "more friendly" format should be made.

## How

Of course, being a proprietary format, .upt files needed to be "reverse engineered" first. Essentially, they are UTF-16LE encoded XML files with a custom extension. They can contain folder(s) with *n* levels of subfolders, with questions and answers in any of them. I will not go into details about the format itself, I believe the code documents itself in this regard as much as necessary.

However, it's worth noting that only a subset of the format is being used in this conversion, both horizontally (ie. the numbed of questions types that may be converted) and vertically (ie. the number of features on any one question). This is due to the ultimate business requirement, the output format.

### CLI tool

TODO.

### Output format

The output format of the conversion is flattened in nature, ie. subfolders are recursively processed. Folders themselves, however, are preserved.

#### Types

##### Collection

This is the top-level object that is generated

| Property    | Type         | Note          |
| ----------- | ------------ | ------------- |
| `name`    | `string`   | User provided |
| `folders` | `Folder[]` |               |

##### Folder

| Property      | Type           | Note |
| ------------- | -------------- | ---- |
| `name`      | `string`     |      |
| `questions` | `Question[]` |      |

##### Question

| Property            | Type              | Note                                                                                  |
| ------------------- | ----------------- | ------------------------------------------------------------------------------------- |
| `text`            | `string`        | The body of the question                                                              |
| `type`            | `QuestionType`  | Enum                                                                                  |
| `possibleAnswers` | `string[]`      | Empty when type is `ExactText`                                                      |
| `possibleOptions` | `string[]`      | Possible options. Only applicable for `MultipleAnswers`, `Table` and `Grouping` |
| `answers`         | `AnswerWrapper` | Despite the naming, it's a single object                                              |

##### AnswerWrapper

| Property         | Type              | Note                        |
| ---------------- | ----------------- | --------------------------- |
| `singleAnswer` | `string \| null` | Deprecated                  |
| `textAnswers`  | `string[]`      | All valid free text answers |
| `answers`      | `Answer[]`      | Non-free text answers       |

##### Answer

| Property        | Type       | Note                                                       |
| --------------- | ---------- | ---------------------------------------------------------- |
| `answerIndex` | `number` | The index of the valid answer                              |
| `optionIndex` | `number` | The index of the matching option, `-1` if not applicable |

##### QuestionType

* `ExactText`
* `SingleAnswer`
* `MultipleAnswers`
* `Table`
* `Grouping`

## Licensing

This project is public for demonstration purposes only. Contact me at [hello@markszente.io](hello@markszente.io) if you are interested in using it.
