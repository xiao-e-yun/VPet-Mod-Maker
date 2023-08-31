import { PetAnimationLoop, PetAnimationSingle } from "@interface";

export const enum AnimationType { Single, Loop, Layer ,Cutsom }
export type PetAnimation = PetAnimationLoop | PetAnimationSingle