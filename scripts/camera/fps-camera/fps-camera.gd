extends Node3D

@export
var player: Player

@export
var camera_speed := 15

@export
var mouse_sensitivity := 2

@export
var max_rotation := 80

@export
var zoom_speed := 70

@export
var zoom_max := 8

@export
var zoom_min := 1

@export
var zoom_start_y := 0.5

@export
var zoom_end_y := 3.5

@export
var light: DirectionalLight3D

@export
var disable_post_processing_in_editor := false

@onready
var pivot := self
var pivot_margin := Vector3.ZERO


@onready
var inner_gimbal := $inner_gimbal

@onready
var spring := $inner_gimbal/SpringArm3D

@onready
var camera := $inner_gimbal/SpringArm3D/PostProcessingCamera

func _ready():
	camera.light = light
	camera.disable_post_processing_in_editor = disable_post_processing_in_editor
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
	pivot_margin = player.car.global_position - pivot.global_position


func _process(delta):
	var zoom := 0
	
	if Input.get_mouse_mode() == Input.MOUSE_MODE_VISIBLE:
		return
	
	if Input.is_action_just_released("zoom_in"):
		zoom -= 1
	elif Input.is_action_just_released("zoom_out"):
		zoom += 1
	
	spring.spring_length = clampf(spring.spring_length + (zoom * zoom_speed * delta), zoom_min, zoom_max)
	spring.position.y = lerpf(zoom_start_y, zoom_end_y, spring.spring_length / zoom_max)


func _physics_process(delta):
	if Input.get_mouse_mode() == Input.MOUSE_MODE_CAPTURED:
		pivot.global_position = pivot.global_position.lerp(player.car.global_position - pivot_margin, delta * camera_speed)


func _input(event):
	if event is InputEventMouseMotion and Input.get_mouse_mode() == Input.MOUSE_MODE_CAPTURED:
		inner_gimbal.rotate_x(deg_to_rad(event.relative.y * mouse_sensitivity))
		inner_gimbal.rotation.x = clampf(inner_gimbal.rotation.x, deg_to_rad(-max_rotation), deg_to_rad(max_rotation))
		pivot.rotate_y(deg_to_rad(event.relative.x * mouse_sensitivity * -1))
	
	if event.is_action_pressed("show_cursor"):
		Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)
	
	if event.is_action_released("show_cursor"):
		Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
