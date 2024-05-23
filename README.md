# Rantz Spatial2D

A 2d transform system for the [Bevy](https://bevyengine.org/) game engine that supports Transform propogation controls. 

## Core Types

### `Position2D`

The `Position2D` struct represents a 2D point in space. It has two fields: `x` and `y`, representing the Cartesian coordinates of the point.

### `Rotation2D`

The `Rotation2D` struct represents a 2D rotation. It can be used with either radians or degrees.

### `Scale2D`

The `Scale2D` struct represents a 2D scale. It has two fields: `x` and `y`, representing the scale factors for the x and y axes respectively.

### `SpatialBundle2D`

The `SpatialBundle2D` struct represents the spatial state of a transform in 2D. It contains a `Position2D`, `Rotation2D`, and `Scale2D`. It provides a convenient way to store and manipulate the spatial state of your game objects.

When you add a `SpatialBundle2D` to an entity, it will also automatically add a `SpatialBundle`, making it easy to use the spatial 2D system with Bevy's built-in transform system.

### `DrawOrder`

The `DrawOrder` struct is used to specify the draw layering of entities in your game. It is a wrapper over an f32, with higher values drawing above lower values. 

## Propogation Types
The PositionPropogation and RotationPropogation enums are used to control how the spatial components of an entity are propogated to its children.

### PositionPropogation
The PositionPropogation enum has two variants:

**Relative:** *[Default]*The position of a child entity is relative to its parent's position.
**Absolute:** The position of a child entity is absolute and does not take into account its parent's position.

### RotationPropogation
The RotationPropogation enum has two variants:

**Relative:** *[Default]*The rotation of a child entity is relative to its parent's rotation.
**Absolute:** The rotation of a child entity is absolute and does not take into account its parent's rotation.
By default, both PositionPropogation and RotationPropogation are set to Relative.

### ScalePropogation
The ScalePropogation enum has two variants:

**Relative:** *[Default]*The rotation of a child entity is relative to its parent's rotation.
**Absolute:** The rotation of a child entity is absolute and does not take into account its parent's rotation.

## Usage

Add the crate to your `Cargo.toml` and `SpatialPlugin2D` to your app.

