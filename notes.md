# A E S T H E T I C S

## Scope

3D Graphics, and basic Keyboard/Mouse interaction. Audio and GUI should be the next extension of the project.

## Structure

Window holds the event loop and display.
passed as a reference to other structures that need access to graphics hardware (display) or os window (events).

Renderer draws Objects.
A Renderer will contain a Shader, and a map of Buffers to Objects (perhaps by ID).

Shader traits are implemented by Shader structs, to match the glsl source code.
trait should be able to set uniforms and vertex data given a specific Object trait.
these methods should return an error if implementation cannot deal with object type

Object traits implemented by Object structs. The traits implemented determine the type of the object.
the traits define the getting and updating of internal data.
initalizing the data is left up to the implementing struct.


> Shader implements traits to specify which vertex format and uniforms they accept.
> Object also implements traits to specify what parameters they have (light, position, albedo, texture, etc).

> Renderer should work with the traits, not specific implementations, this will be done by the shaders and objects.

> draw function accepts an Object then matches .

