@tool
extends Node3D
class_name PostProcessingCamera

@export
var light: DirectionalLight3D

@export
var disable_post_processing_in_editor = false

@onready
var post_processing: MeshInstance3D = $Camera3D/PostProcessing

func _process(_delta):
	post_processing.visible = false#= (!Engine.is_editor_hint() || !disable_post_processing_in_editor)
	
	if light != null:
		post_processing.mesh.surface_get_material(0).set_shader_parameter('light_direction', -light.global_basis.z)

