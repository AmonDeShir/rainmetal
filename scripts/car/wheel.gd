extends RayCast3D

enum STEERING_TYPE {NONE, LEFT, RIGHT }

@export
var use_as_steering: STEERING_TYPE = STEERING_TYPE.NONE
@export
var use_as_traction: bool = true
@export
var tire_grip: float = 2.0

@onready
var car: Car = get_parent().get_parent()

var previus_spring_length := 0.0

func _ready():
	add_exception(car)

  
func _process(delta: float):
	if use_as_steering != STEERING_TYPE.NONE:
		var angle := lerpf(rotation.y, car.steering_angle, car.steering_speed) * delta
		rotation.y = angle
		
		if car.debug:
			$debug_mesh.show()
		
	
func _physics_process(delta: float):
	if is_colliding():
		var collision_point := get_collision_point()
		
		suspension(delta, collision_point)
		acceleration(collision_point)
		friction(collision_point)


func suspension(delta: float, raycast_dest: Vector3):
		var susp_direction := global_basis.y
		
		var raycast_origin := global_position
		var distance := raycast_dest.distance_to(raycast_origin)
		
		var spring_length := clampf(distance - car.wheel_radius, 0, car.suspension_rest_distance)
		var spring_force := car.spring_strength * (car.suspension_rest_distance - spring_length)
		var spring_velocity := (previus_spring_length - spring_length) / delta
		
		var damper_force := car.spring_damper * spring_velocity
		var suspension_force := basis.y * (spring_force + damper_force)
		
		previus_spring_length = spring_length 
		
		var point := get_wheel_point(raycast_dest)
		
		car.apply_force(susp_direction * suspension_force, point - car.global_position)
		
		if car.debug:
			# Suspension (spring)
			DebugDraw3D.draw_arrow(global_position, to_global(position + Vector3(-position.x, (suspension_force.y / 2), -position.z)), Color.GREEN, 0.1, true)
			
			# Raycast
			DebugDraw3D.draw_line_hit_offset(global_position, to_global(position + Vector3(-position.x, -1, -position.z)), true, distance, 0.2, Color.RED, Color.RED)


func acceleration(collision_point: Vector3):
	if not use_as_traction:
		return
	
	var direction := -global_basis.z
	
	var torque := car.axel_input * car.engine_power
	var point := get_wheel_point(collision_point)
	
	car.apply_force(direction * torque, point - car.global_position)
	
	if car.debug:
		DebugDraw3D.draw_arrow(point, point + (direction * torque), Color.BLUE_VIOLET, 0.1, true)


func get_wheel_point(collision_point: Vector3) -> Vector3:
	return collision_point + Vector3(0, car.wheel_radius, 0)


func friction(collision_point: Vector3):
	var direction := global_basis.z
	var tire_word_vel := get_point_velocity(global_position)
	
	# add here more advance friction!
	var force = direction.dot(tire_word_vel) * (car.mass / 10)
	
	car.apply_force(-direction * force, collision_point - car.global_position)
	
	if car.debug:
		var point = get_wheel_point(collision_point)
		DebugDraw3D.draw_arrow(point, point + (-direction * force), Color.SANDY_BROWN, 0.1, true)


func get_point_velocity(point: Vector3) -> Vector3:
	return car.linear_velocity + car.angular_velocity.cross(point - car.global_position)
