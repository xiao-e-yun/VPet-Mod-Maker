import { convertFileSrc } from '@tauri-apps/api/tauri';
import { __VLS_publicComponent, __VLS_internalComponent, __VLS_componentsOption, __VLS_name, foods, setImage, createNewFood, infoData, setDefaultImage } from './food.vue';

function __VLS_template() {
let __VLS_ctx!: InstanceType<__VLS_PickNotAny<typeof __VLS_publicComponent, new () => {}>> & InstanceType<__VLS_PickNotAny<typeof __VLS_internalComponent, new () => {}>> & {
$style: Record<string, string> & __VLS_Prettify<{} &
{ 'food': string; } &
{ 'base': string; } &
{ 'title': string; } &
{ 'image': string; } &
{ 'status': string; } &
{ 'image': string; }>;
};
/* Components */
let __VLS_otherComponents!: NonNullable<typeof __VLS_internalComponent extends { components: infer C; } ? C : {}> & typeof __VLS_componentsOption;
let __VLS_own!: __VLS_SelfComponent<typeof __VLS_name, typeof __VLS_internalComponent & typeof __VLS_publicComponent & (new () => { $slots: typeof __VLS_slots; })>;
let __VLS_localComponents!: typeof __VLS_otherComponents & Omit<typeof __VLS_own, keyof typeof __VLS_otherComponents>;
let __VLS_components!: typeof __VLS_localComponents & __VLS_GlobalComponents & typeof __VLS_ctx;
/* Style Scoped */
type __VLS_StyleScopedClasses = {};
let __VLS_styleScopedClasses!: __VLS_StyleScopedClasses | keyof __VLS_StyleScopedClasses | (keyof __VLS_StyleScopedClasses)[];
/* CSS variable injection */
/* CSS variable injection end */
let __VLS_resolvedLocalAndGlobalComponents!: {};
({} as __VLS_IntrinsicElements).h1; ({} as __VLS_IntrinsicElements).h1;
({} as __VLS_IntrinsicElements).div; ({} as __VLS_IntrinsicElements).div; ({} as __VLS_IntrinsicElements).div; ({} as __VLS_IntrinsicElements).div; ({} as __VLS_IntrinsicElements).div; ({} as __VLS_IntrinsicElements).div; ({} as __VLS_IntrinsicElements).div; ({} as __VLS_IntrinsicElements).div;
({} as __VLS_IntrinsicElements).input; ({} as __VLS_IntrinsicElements).input; ({} as __VLS_IntrinsicElements).input; ({} as __VLS_IntrinsicElements).input; ({} as __VLS_IntrinsicElements).input; ({} as __VLS_IntrinsicElements).input; ({} as __VLS_IntrinsicElements).input; ({} as __VLS_IntrinsicElements).input; ({} as __VLS_IntrinsicElements).input;
({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label; ({} as __VLS_IntrinsicElements).label;
({} as __VLS_IntrinsicElements).img; ({} as __VLS_IntrinsicElements).img;
({} as __VLS_IntrinsicElements).select; ({} as __VLS_IntrinsicElements).select;
({} as __VLS_IntrinsicElements).option; ({} as __VLS_IntrinsicElements).option; ({} as __VLS_IntrinsicElements).option; ({} as __VLS_IntrinsicElements).option; ({} as __VLS_IntrinsicElements).option; ({} as __VLS_IntrinsicElements).option; ({} as __VLS_IntrinsicElements).option; ({} as __VLS_IntrinsicElements).option; ({} as __VLS_IntrinsicElements).option; ({} as __VLS_IntrinsicElements).option;
({} as __VLS_IntrinsicElements).button; ({} as __VLS_IntrinsicElements).button; ({} as __VLS_IntrinsicElements).button; ({} as __VLS_IntrinsicElements).button;
({} as __VLS_IntrinsicElements).h2; ({} as __VLS_IntrinsicElements).h2;
{
const __VLS_0 = ({} as __VLS_IntrinsicElements)["h1"];
const __VLS_1 = __VLS_asFunctionalComponent(__VLS_0, {});
({} as __VLS_IntrinsicElements).h1;
({} as __VLS_IntrinsicElements).h1;
const __VLS_2 = __VLS_1({ ...{}, }, ...__VLS_functionalComponentArgsRest(__VLS_1));
const __VLS_3 = __VLS_pickFunctionalComponentCtx(__VLS_0, __VLS_2)!;
}
{
const __VLS_4 = ({} as __VLS_IntrinsicElements)["div"];
const __VLS_5 = __VLS_asFunctionalComponent(__VLS_4, {});
({} as __VLS_IntrinsicElements).div;
({} as __VLS_IntrinsicElements).div;
const __VLS_6 = __VLS_5({ ...{}, }, ...__VLS_functionalComponentArgsRest(__VLS_5));
const __VLS_7 = __VLS_pickFunctionalComponentCtx(__VLS_4, __VLS_6)!;
for (const [food, index] of __VLS_getVForSourceType((__VLS_ctx.foods)!)) {
{
const __VLS_8 = ({} as __VLS_IntrinsicElements)["div"];
const __VLS_9 = __VLS_asFunctionalComponent(__VLS_8, {});
({} as __VLS_IntrinsicElements).div;
({} as __VLS_IntrinsicElements).div;
const __VLS_10 = __VLS_9({ ...{}, class: ((__VLS_ctx.$style.food)), }, ...__VLS_functionalComponentArgsRest(__VLS_9));
const __VLS_11 = __VLS_pickFunctionalComponentCtx(__VLS_8, __VLS_10)!;
{
const __VLS_12 = ({} as __VLS_IntrinsicElements)["div"];
const __VLS_13 = __VLS_asFunctionalComponent(__VLS_12, {});
({} as __VLS_IntrinsicElements).div;
({} as __VLS_IntrinsicElements).div;
const __VLS_14 = __VLS_13({ ...{}, class: ((__VLS_ctx.$style.base)), }, ...__VLS_functionalComponentArgsRest(__VLS_13));
const __VLS_15 = __VLS_pickFunctionalComponentCtx(__VLS_12, __VLS_14)!;
{
const __VLS_16 = ({} as __VLS_IntrinsicElements)["input"];
const __VLS_17 = __VLS_asFunctionalComponent(__VLS_16, {});
({} as __VLS_IntrinsicElements).input;
const __VLS_18 = __VLS_17({ ...{}, type: ("text"), placeholder: ("食物名稱"), value: ((food.name)), class: ((__VLS_ctx.$style.title)), }, ...__VLS_functionalComponentArgsRest(__VLS_17));
const __VLS_19 = __VLS_pickFunctionalComponentCtx(__VLS_16, __VLS_18)!;
}
{
const __VLS_20 = ({} as __VLS_IntrinsicElements)["label"];
const __VLS_21 = __VLS_asFunctionalComponent(__VLS_20, {});
({} as __VLS_IntrinsicElements).label;
({} as __VLS_IntrinsicElements).label;
const __VLS_22 = __VLS_21({ ...{}, }, ...__VLS_functionalComponentArgsRest(__VLS_21));
const __VLS_23 = __VLS_pickFunctionalComponentCtx(__VLS_20, __VLS_22)!;
{
const __VLS_24 = ({} as __VLS_IntrinsicElements)["input"];
const __VLS_25 = __VLS_asFunctionalComponent(__VLS_24, {});
({} as __VLS_IntrinsicElements).input;
const __VLS_26 = __VLS_25({ ...{}, type: ("text"), value: ((food.desc)), placeholder: ("描述"), }, ...__VLS_functionalComponentArgsRest(__VLS_25));
const __VLS_27 = __VLS_pickFunctionalComponentCtx(__VLS_24, __VLS_26)!;
}
}
{
const __VLS_28 = ({} as __VLS_IntrinsicElements)["img"];
const __VLS_29 = __VLS_asFunctionalComponent(__VLS_28, {});
({} as __VLS_IntrinsicElements).img;
const __VLS_30 = __VLS_29({ ...{ onClick: {} as any, }, class: ((__VLS_ctx.$style.image)), src: ((food.image ? __VLS_ctx.convertFileSrc(food.image) : 'data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjwvc3ZnPg==')), }, ...__VLS_functionalComponentArgsRest(__VLS_29));
const __VLS_31 = __VLS_pickFunctionalComponentCtx(__VLS_28, __VLS_30)!;
let __VLS_32 = { 'click': __VLS_pickEvent(__VLS_31.emit!, 'click' as const, __VLS_componentProps(__VLS_29, __VLS_30).onClick) };
__VLS_32 = {
click: $event => {
__VLS_ctx.setImage(food);
}
};
}
}
{
const __VLS_33 = ({} as __VLS_IntrinsicElements)["div"];
const __VLS_34 = __VLS_asFunctionalComponent(__VLS_33, {});
({} as __VLS_IntrinsicElements).div;
({} as __VLS_IntrinsicElements).div;
const __VLS_35 = __VLS_34({ ...{}, class: ((__VLS_ctx.$style.status)), }, ...__VLS_functionalComponentArgsRest(__VLS_34));
const __VLS_36 = __VLS_pickFunctionalComponentCtx(__VLS_33, __VLS_35)!;
{
const __VLS_37 = ({} as __VLS_IntrinsicElements)["label"];
const __VLS_38 = __VLS_asFunctionalComponent(__VLS_37, {});
({} as __VLS_IntrinsicElements).label;
({} as __VLS_IntrinsicElements).label;
const __VLS_39 = __VLS_38({ ...{}, }, ...__VLS_functionalComponentArgsRest(__VLS_38));
const __VLS_40 = __VLS_pickFunctionalComponentCtx(__VLS_37, __VLS_39)!;
{
const __VLS_41 = ({} as __VLS_IntrinsicElements)["select"];
const __VLS_42 = __VLS_asFunctionalComponent(__VLS_41, {});
({} as __VLS_IntrinsicElements).select;
({} as __VLS_IntrinsicElements).select;
const __VLS_43 = __VLS_42({ ...{}, value: ((food.type)), }, ...__VLS_functionalComponentArgsRest(__VLS_42));
const __VLS_44 = __VLS_pickFunctionalComponentCtx(__VLS_41, __VLS_43)!;
{
const __VLS_45 = ({} as __VLS_IntrinsicElements)["option"];
const __VLS_46 = __VLS_asFunctionalComponent(__VLS_45, {});
({} as __VLS_IntrinsicElements).option;
({} as __VLS_IntrinsicElements).option;
const __VLS_47 = __VLS_46({ ...{}, value: ("Meal"), }, ...__VLS_functionalComponentArgsRest(__VLS_46));
const __VLS_48 = __VLS_pickFunctionalComponentCtx(__VLS_45, __VLS_47)!;
}
{
const __VLS_49 = ({} as __VLS_IntrinsicElements)["option"];
const __VLS_50 = __VLS_asFunctionalComponent(__VLS_49, {});
({} as __VLS_IntrinsicElements).option;
({} as __VLS_IntrinsicElements).option;
const __VLS_51 = __VLS_50({ ...{}, value: ("Snack"), }, ...__VLS_functionalComponentArgsRest(__VLS_50));
const __VLS_52 = __VLS_pickFunctionalComponentCtx(__VLS_49, __VLS_51)!;
}
{
const __VLS_53 = ({} as __VLS_IntrinsicElements)["option"];
const __VLS_54 = __VLS_asFunctionalComponent(__VLS_53, {});
({} as __VLS_IntrinsicElements).option;
({} as __VLS_IntrinsicElements).option;
const __VLS_55 = __VLS_54({ ...{}, value: ("Drink"), }, ...__VLS_functionalComponentArgsRest(__VLS_54));
const __VLS_56 = __VLS_pickFunctionalComponentCtx(__VLS_53, __VLS_55)!;
}
{
const __VLS_57 = ({} as __VLS_IntrinsicElements)["option"];
const __VLS_58 = __VLS_asFunctionalComponent(__VLS_57, {});
({} as __VLS_IntrinsicElements).option;
({} as __VLS_IntrinsicElements).option;
const __VLS_59 = __VLS_58({ ...{}, value: ("Functional"), }, ...__VLS_functionalComponentArgsRest(__VLS_58));
const __VLS_60 = __VLS_pickFunctionalComponentCtx(__VLS_57, __VLS_59)!;
}
{
const __VLS_61 = ({} as __VLS_IntrinsicElements)["option"];
const __VLS_62 = __VLS_asFunctionalComponent(__VLS_61, {});
({} as __VLS_IntrinsicElements).option;
({} as __VLS_IntrinsicElements).option;
const __VLS_63 = __VLS_62({ ...{}, value: ("Drug"), }, ...__VLS_functionalComponentArgsRest(__VLS_62));
const __VLS_64 = __VLS_pickFunctionalComponentCtx(__VLS_61, __VLS_63)!;
}
}
}
{
const __VLS_65 = ({} as __VLS_IntrinsicElements)["label"];
const __VLS_66 = __VLS_asFunctionalComponent(__VLS_65, {});
({} as __VLS_IntrinsicElements).label;
({} as __VLS_IntrinsicElements).label;
const __VLS_67 = __VLS_66({ ...{}, }, ...__VLS_functionalComponentArgsRest(__VLS_66));
const __VLS_68 = __VLS_pickFunctionalComponentCtx(__VLS_65, __VLS_67)!;
{
const __VLS_69 = ({} as __VLS_IntrinsicElements)["input"];
const __VLS_70 = __VLS_asFunctionalComponent(__VLS_69, {});
({} as __VLS_IntrinsicElements).input;
const __VLS_71 = __VLS_70({ ...{}, type: ("number"), }, ...__VLS_functionalComponentArgsRest(__VLS_70));
const __VLS_72 = __VLS_pickFunctionalComponentCtx(__VLS_69, __VLS_71)!;
(food.price);
}
}
{
const __VLS_73 = ({} as __VLS_IntrinsicElements)["label"];
const __VLS_74 = __VLS_asFunctionalComponent(__VLS_73, {});
({} as __VLS_IntrinsicElements).label;
({} as __VLS_IntrinsicElements).label;
const __VLS_75 = __VLS_74({ ...{}, }, ...__VLS_functionalComponentArgsRest(__VLS_74));
const __VLS_76 = __VLS_pickFunctionalComponentCtx(__VLS_73, __VLS_75)!;
{
const __VLS_77 = ({} as __VLS_IntrinsicElements)["input"];
const __VLS_78 = __VLS_asFunctionalComponent(__VLS_77, {});
({} as __VLS_IntrinsicElements).input;
const __VLS_79 = __VLS_78({ ...{}, type: ("number"), }, ...__VLS_functionalComponentArgsRest(__VLS_78));
const __VLS_80 = __VLS_pickFunctionalComponentCtx(__VLS_77, __VLS_79)!;
(food.exp);
}
}
{
const __VLS_81 = ({} as __VLS_IntrinsicElements)["label"];
const __VLS_82 = __VLS_asFunctionalComponent(__VLS_81, {});
({} as __VLS_IntrinsicElements).label;
({} as __VLS_IntrinsicElements).label;
const __VLS_83 = __VLS_82({ ...{}, }, ...__VLS_functionalComponentArgsRest(__VLS_82));
const __VLS_84 = __VLS_pickFunctionalComponentCtx(__VLS_81, __VLS_83)!;
{
const __VLS_85 = ({} as __VLS_IntrinsicElements)["input"];
const __VLS_86 = __VLS_asFunctionalComponent(__VLS_85, {});
({} as __VLS_IntrinsicElements).input;
const __VLS_87 = __VLS_86({ ...{}, type: ("number"), }, ...__VLS_functionalComponentArgsRest(__VLS_86));
const __VLS_88 = __VLS_pickFunctionalComponentCtx(__VLS_85, __VLS_87)!;
(food.strengthFood);
}
}
{
const __VLS_89 = ({} as __VLS_IntrinsicElements)["label"];
const __VLS_90 = __VLS_asFunctionalComponent(__VLS_89, {});
({} as __VLS_IntrinsicElements).label;
({} as __VLS_IntrinsicElements).label;
const __VLS_91 = __VLS_90({ ...{}, }, ...__VLS_functionalComponentArgsRest(__VLS_90));
const __VLS_92 = __VLS_pickFunctionalComponentCtx(__VLS_89, __VLS_91)!;
{
const __VLS_93 = ({} as __VLS_IntrinsicElements)["input"];
const __VLS_94 = __VLS_asFunctionalComponent(__VLS_93, {});
({} as __VLS_IntrinsicElements).input;
const __VLS_95 = __VLS_94({ ...{}, type: ("number"), }, ...__VLS_functionalComponentArgsRest(__VLS_94));
const __VLS_96 = __VLS_pickFunctionalComponentCtx(__VLS_93, __VLS_95)!;
(food.feeling);
}
}
{
const __VLS_97 = ({} as __VLS_IntrinsicElements)["label"];
const __VLS_98 = __VLS_asFunctionalComponent(__VLS_97, {});
({} as __VLS_IntrinsicElements).label;
({} as __VLS_IntrinsicElements).label;
const __VLS_99 = __VLS_98({ ...{}, }, ...__VLS_functionalComponentArgsRest(__VLS_98));
const __VLS_100 = __VLS_pickFunctionalComponentCtx(__VLS_97, __VLS_99)!;
{
const __VLS_101 = ({} as __VLS_IntrinsicElements)["input"];
const __VLS_102 = __VLS_asFunctionalComponent(__VLS_101, {});
({} as __VLS_IntrinsicElements).input;
const __VLS_103 = __VLS_102({ ...{}, type: ("number"), }, ...__VLS_functionalComponentArgsRest(__VLS_102));
const __VLS_104 = __VLS_pickFunctionalComponentCtx(__VLS_101, __VLS_103)!;
(food.strengthDrink);
}
}
{
const __VLS_105 = ({} as __VLS_IntrinsicElements)["label"];
const __VLS_106 = __VLS_asFunctionalComponent(__VLS_105, {});
({} as __VLS_IntrinsicElements).label;
({} as __VLS_IntrinsicElements).label;
const __VLS_107 = __VLS_106({ ...{}, }, ...__VLS_functionalComponentArgsRest(__VLS_106));
const __VLS_108 = __VLS_pickFunctionalComponentCtx(__VLS_105, __VLS_107)!;
{
const __VLS_109 = ({} as __VLS_IntrinsicElements)["input"];
const __VLS_110 = __VLS_asFunctionalComponent(__VLS_109, {});
({} as __VLS_IntrinsicElements).input;
const __VLS_111 = __VLS_110({ ...{}, type: ("number"), }, ...__VLS_functionalComponentArgsRest(__VLS_110));
const __VLS_112 = __VLS_pickFunctionalComponentCtx(__VLS_109, __VLS_111)!;
(food.health);
}
}
{
const __VLS_113 = ({} as __VLS_IntrinsicElements)["label"];
const __VLS_114 = __VLS_asFunctionalComponent(__VLS_113, {});
({} as __VLS_IntrinsicElements).label;
({} as __VLS_IntrinsicElements).label;
const __VLS_115 = __VLS_114({ ...{}, }, ...__VLS_functionalComponentArgsRest(__VLS_114));
const __VLS_116 = __VLS_pickFunctionalComponentCtx(__VLS_113, __VLS_115)!;
{
const __VLS_117 = ({} as __VLS_IntrinsicElements)["input"];
const __VLS_118 = __VLS_asFunctionalComponent(__VLS_117, {});
({} as __VLS_IntrinsicElements).input;
const __VLS_119 = __VLS_118({ ...{}, type: ("number"), }, ...__VLS_functionalComponentArgsRest(__VLS_118));
const __VLS_120 = __VLS_pickFunctionalComponentCtx(__VLS_117, __VLS_119)!;
(food.likability);
}
}
{
const __VLS_121 = ({} as __VLS_IntrinsicElements)["button"];
const __VLS_122 = __VLS_asFunctionalComponent(__VLS_121, {});
({} as __VLS_IntrinsicElements).button;
({} as __VLS_IntrinsicElements).button;
const __VLS_123 = __VLS_122({ ...{ onClick: {} as any, }, }, ...__VLS_functionalComponentArgsRest(__VLS_122));
const __VLS_124 = __VLS_pickFunctionalComponentCtx(__VLS_121, __VLS_123)!;
let __VLS_125 = { 'click': __VLS_pickEvent(__VLS_124.emit!, 'click' as const, __VLS_componentProps(__VLS_122, __VLS_123).onClick) };
__VLS_125 = {
click: $event => {
__VLS_ctx.foods.splice(index);
}
};
}
}
}
// @ts-ignore
[foods, $style, $style, $style, $style, convertFileSrc, setImage, $style, foods,];
}
{
const __VLS_126 = ({} as __VLS_IntrinsicElements)["button"];
const __VLS_127 = __VLS_asFunctionalComponent(__VLS_126, {});
({} as __VLS_IntrinsicElements).button;
({} as __VLS_IntrinsicElements).button;
const __VLS_128 = __VLS_127({ ...{ onClick: {} as any, }, class: ((__VLS_ctx.$style.create)), }, ...__VLS_functionalComponentArgsRest(__VLS_127));
const __VLS_129 = __VLS_pickFunctionalComponentCtx(__VLS_126, __VLS_128)!;
let __VLS_130 = { 'click': __VLS_pickEvent(__VLS_129.emit!, 'click' as const, __VLS_componentProps(__VLS_127, __VLS_128).onClick) };
__VLS_130 = {
click: (__VLS_ctx.createNewFood)
};
}
}
{
const __VLS_131 = ({} as __VLS_IntrinsicElements)["h2"];
const __VLS_132 = __VLS_asFunctionalComponent(__VLS_131, {});
({} as __VLS_IntrinsicElements).h2;
({} as __VLS_IntrinsicElements).h2;
const __VLS_133 = __VLS_132({ ...{}, }, ...__VLS_functionalComponentArgsRest(__VLS_132));
const __VLS_134 = __VLS_pickFunctionalComponentCtx(__VLS_131, __VLS_133)!;
}
{
const __VLS_135 = ({} as __VLS_IntrinsicElements)["img"];
const __VLS_136 = __VLS_asFunctionalComponent(__VLS_135, {});
({} as __VLS_IntrinsicElements).img;
const __VLS_137 = __VLS_136({ ...{ onClick: {} as any, }, class: ((__VLS_ctx.$style.image)), src: ((__VLS_ctx.loadImageSrc(__VLS_ctx.infoData.foodImage))), }, ...__VLS_functionalComponentArgsRest(__VLS_136));
const __VLS_138 = __VLS_pickFunctionalComponentCtx(__VLS_135, __VLS_137)!;
let __VLS_139 = { 'click': __VLS_pickEvent(__VLS_138.emit!, 'click' as const, __VLS_componentProps(__VLS_136, __VLS_137).onClick) };
__VLS_139 = {
click: (__VLS_ctx.setDefaultImage)
};
}
if (typeof __VLS_styleScopedClasses === 'object' && !Array.isArray(__VLS_styleScopedClasses)) {
}
var __VLS_slots!: {};
// @ts-ignore
[$style, createNewFood, $style, loadImageSrc, infoData, setDefaultImage,];
return __VLS_slots;
}
