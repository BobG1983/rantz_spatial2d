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
The following enums are used to control how the spatial components of an entity are propogated to its children.

### `PositionPropogation`
The `PositionPropogation` enum has two variants:

**Relative:** **[Default]**The position of a child entity is relative to its parent's position.

**Absolute:** The position of a child entity is absolute and does not take into account its parent's position.

### `RotationPropogation`
The `RotationPropogation` enum has two variants:

**Relative:** **[Default]**The rotation of a child entity is relative to its parent's rotation.

**Absolute:** The rotation of a child entity is absolute and does not take into account its parent's rotation.
By default, both PositionPropogation and RotationPropogation are set to Relative.

### `ScalePropogation`
The `ScalePropogation` enum has two variants:

**Relative:** **[Default]**The rotation of a child entity is relative to its parent's rotation.

**Absolute:** The rotation of a child entity is absolute and does not take into account its parent's rotation.

### Compasses

In addition to the above, and not included in the SpatialBundle2D are `Compass`, `CompassRose`, and `CompassHalfwinds`. It is very common in 2D games to need to know the compass direction a given entity is facing in order to choose which sprite to draw. These enums can be included as a component on anything with a Rotation2D, and as part of transform propogation, there values will be updated to match the objects rotation.

## Feature Flags

* `bevy` - Adds systems to update Bevys internal transform based on the contents of `SpatialBundle2D`, and a plugin to manage said systems, register types, etc.
* `serde` - Adds impls for serde:Serialize and serde:Deserialize

**Default:** `bevy`

## Usage

Add the crate to your `Cargo.toml`

If using bevy, add `SpatialPlugin2D` to your app.

