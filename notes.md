# A E S T H E T I C S

## Scope

3D Graphics, and basic Keyboard/Mouse interaction. Audio and GUI should be the next extension of the project.

## Structure

Event loop and display have convenience methods to create them but they are treated as separate entities.
References to them (are resources/are passed to constructors)

Renderers draw Objects.
A Renderer will contain a Shader, and a map of Buffers to Entities. it will fetch entities that have the components it
needs/knows how to deal with.
