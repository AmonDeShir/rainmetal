extends Camera3D

@export
var player: Player

@export
var zoom_speed = 70

@export
var zoom_max = 15

@export
var zoom_min = 3.5


func _ready():
	size = zoom_min


func _process(delta):
	var input = 0
	
	if Input.is_action_just_released("zoom_in"):
		input += 1
	
	elif Input.is_action_just_released("zoom_out"):
		input -= 1
	
	size = clampf(size + (input * zoom_speed * delta), zoom_min, zoom_max)
