
Desktop
 - workspaces: Workspace[]

Trait AbstractContainer Impl to Container | Workspace
  - Shape(x, y, width, height)

Enum Container:
  - Container::Window | Container::Layout | Container:: Workspace

Workspace
 - containers: Container[]

Container::Layout
 - Layout
 - containers: Container[]

Container::Window
 - Window


