extends RigidBody3D
class_name Car

@export 
var suspension_rest_distance: float = 0.5
@export 
var spring_strength: float = 10
@export 
var spring_damper: float = 1.0
@export 
var wheel_radius: float = 0.33

@export
var engine_power: float

@export
var max_steering_angle: float = 30.0
@export
var steering_speed: float = 0.3

@export
var debug: bool = false

var axel_input: float
var steering_input: float
var steering_angle: float

func _process(_delta):
	axel_input = Input.get_axis("backward", "forward")
	steering_input = Input.get_axis("turn_right", "turn_left")
	steering_angle = max_steering_angle * steering_input
