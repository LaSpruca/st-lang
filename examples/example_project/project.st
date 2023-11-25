using Std::Build::ProjectDescriptor;

func project () (ProjectDescriptor) {
    ProjectDescriptor::init |= project
    |> .{name} "ExampleProject"
    |> .{bin} {name: "example", src: "./src" }
}