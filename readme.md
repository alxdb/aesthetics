# A E S T H E T I C S

## Scope

3D Graphics, and basic Keyboard/Mouse interaction. Audio and GUI should be the next extension of the project.

## Structure

Event loop and display have convenience methods to create them but they are treated as separate entities.
References to them are passed to constructors

Renderers draw Entities with specific Components.
A Renderer will contain a Shader, and a map of Entities to Buffers. it will fetch entities that have the components it
needs/knows how to deal with.
