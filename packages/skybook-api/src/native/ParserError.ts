// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Category } from "./Category";
import type { MetaValue } from "./MetaValue";

/**
 * Error type for the parser
 */
export type ParserError =
    | { type: "Unexpected"; data: string }
    | { type: "SyntaxUnexpected" }
    | { type: "SyntaxUnexpectedEof" }
    | { type: "InvalidItem"; data: string }
    | { type: "InvalidEmptyItem" }
    | { type: "InvalidItemAmount" }
    | { type: "IntFormat"; data: string }
    | { type: "IntRange"; data: string }
    | { type: "FloatFormat"; data: string }
    | { type: "UnusedMetaKey"; data: string }
    | { type: "InvalidMetaValue"; data: [string, MetaValue] }
    | { type: "RequiredMetaValue"; data: string }
    | { type: "InvalidWeaponModifier"; data: string }
    | { type: "InvalidCookEffect"; data: string }
    | { type: "TooManyIngredients" }
    | { type: "InvalidArmorStarNum"; data: number }
    | { type: "InvalidSlot"; data: number }
    | { type: "InvalidTimesClause"; data: number }
    | { type: "InvalidTrial"; data: string }
    | { type: "InvalidCategory"; data: Category }
    | { type: "InvalidCategoryName"; data: string }
    | { type: "InvalidInventoryRow"; data: number }
    | { type: "InvalidInventoryCol"; data: number }
    | { type: "UnusedItemPosition" }
    | { type: "InvalidStringLength"; data: number }
    | { type: "GdtTypeNotSet" }
    | { type: "GdtTypeConflict" }
    | { type: "GdtInvalidIndex"; data: number }
    | { type: "GdtMissingVecComp" }
    | { type: "InvalidEquipmentSlotNum"; data: [Category, number] };
