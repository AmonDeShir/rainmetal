extends RigidBody3D
class_name Car

const MS_TO_KMS := 3.6

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
var steering_speed: float = 10

@export
var speedometer: Speedometer

@export
var debug: bool = false

var axel_input: float
var steering_input: float
var steering_angle: float


func _ready():
	max_steering_angle = deg_to_rad(max_steering_angle)
	set_debug(debug)


func _process(_delta):
	axel_input = Input.get_axis("backward", "forward")
	steering_input = Input.get_axis("turn_right", "turn_left")
	steering_angle = max_steering_angle * steering_input
	
	if speedometer != null:
		speedometer.set_speed(get_car_speed_kms())


func _physics_process(delta):
	unflip(delta)


func unflip(delta: float):
	if rotation.z < deg_to_rad(30):
		return
	
	var rot_speed = lerpf(0.001, 0.01, rad_to_deg(rotation.z)/90) * mass * delta
	rotation.z = lerp_angle(rotation.z, 0,  rot_speed)


func _input(event):
	if (event.is_action("debug") and event.is_released()):
		debug = !debug
		set_debug(debug, self)


func set_debug(value: bool, node: Node3D = self):
	if node == self:
		debug = value
	
	for child in node.get_children():
		if child.name == "mesh":
			child.visible = !value
		elif child.name == "debug_mesh":
			child.visible = value
		else:
			set_debug(value, child)


func get_car_speed_ms():
	return abs(linear_velocity.z)

func get_car_speed_kms():
	return abs(linear_velocity.z * MS_TO_KMS)
