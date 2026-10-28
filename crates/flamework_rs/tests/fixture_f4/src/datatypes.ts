import { Flamework } from "@flamework-experimental/core";

export const vector3 = Flamework.createSerializer<Vector3>();
export const vector2 = Flamework.createSerializer<Vector2>();
export const vector3int16 = Flamework.createSerializer<Vector3int16>();
export const vector2int16 = Flamework.createSerializer<Vector2int16>();
export const color3 = Flamework.createSerializer<Color3>();
export const udim = Flamework.createSerializer<UDim>();
export const udim2 = Flamework.createSerializer<UDim2>();
export const numberRange = Flamework.createSerializer<NumberRange>();
export const rect = Flamework.createSerializer<Rect>();
export const brickColor = Flamework.createSerializer<BrickColor>();
export const cframe = Flamework.createSerializer<CFrame>();
export const material = Flamework.createSerializer<Enum.Material>();
export const keyCode = Flamework.createSerializer<Enum.KeyCode | undefined>();
export const someMaterials = Flamework.createSerializer<Enum.Material.Plastic | Enum.Material.Wood>();
export const oneMaterial = Flamework.createSerializer<Enum.Material.Plastic>();
export const colors = Flamework.createSerializer<Array<Color3 | BrickColor>>();
export const placements = Flamework.createSerializer<Map<Instance, CFrame>>();
