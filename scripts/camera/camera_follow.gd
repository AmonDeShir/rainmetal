extends Node3D

@export
var target: Player
var distance: Vector3

@export
var rotation_speed = 0.005

var rotate_camera = false

func _ready():
	distance = target.car.position - position


func _process(delta):
	var z = position.z
	
	position = lerp(position, target.car.position - distance, delta)
	position.z = z

func _input(event):
	if event.is_action_pressed("rotate_camera"):
		rotate_camera = true
	
	if event.is_action_released("rotate_camera"):
		rotate_camera = false
	
	if rotate_camera and event is InputEventMouseMotion:
			rotate_object_local(Vector3.UP, event.relative.x * rotation_speed)
