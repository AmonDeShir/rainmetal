extends RigidBody3D
class_name Car

const MS_TO_KMS := 3.6
const HP_TO_NMS := 745


@export 
# where spring like to be
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
var max_speed: float

@export
var max_steering_angle: Curve
@export
var steering_speed: float = 10

@export
var speedometer: Speedometer

@export
var debug: bool = false

var axel_input: float
var steering_input: float
var steering_angle: float

var stuck_timer = 0.1
var collider: PhysicsBody3D

func _ready():
	engine_power = engine_power * (HP_TO_NMS / 7.3)
	set_debug(debug)


func _process(_delta):
	axel_input = Input.get_axis("backward", "forward")
	steering_input = Input.get_axis("turn_right", "turn_left")
	steering_angle = deg_to_rad(max_steering_angle.sample(get_car_speed_kms()/max_speed)) * steering_input
	
	if speedometer != null:
		speedometer.set_speed(get_car_speed_kms())


func _physics_process(delta):
	unflip(delta)
	unstuck(delta)

func unflip(delta: float):
	if rotation.z < deg_to_rad(30):
		return
	
	var rot_speed = lerpf(0.001, 0.01, rad_to_deg(rotation.z)/90) * mass * delta
	rotation.z = lerp_angle(rotation.z, 0,  rot_speed)


func unstuck(delta: float):
	var stuck = true
	
	if collider == null:
		return
	
	for part in get_children():
		for child in part.get_children():
			if child.name == "wheels":
				var first = child.get_children().front().get_distance_to_ground()
				
				for wheel in child.get_children():
					if abs(first - wheel.get_distance_to_ground()) > 0.1:
						stuck = false
	
	if stuck:
		stuck_timer -= delta
	else:
		stuck_timer = 0.1
	
	if stuck_timer <= 0:
		stuck_timer = 0.1
		translate(Vector3(0, 0.5, 0))


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


func _on_body_entered(body):
	print("colliding with: ", body)
	collider = body

func _on_body_exited(body):
	print("not colliding with: ", body)
	if collider == body:
		collider = null
