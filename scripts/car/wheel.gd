extends RayCast3D

enum STEERING_TYPE {NONE, LEFT, RIGHT }

@export
var use_as_steering: STEERING_TYPE = STEERING_TYPE.NONE
@export
var use_as_traction: bool = true
@export
var tire_grip: float = 2.0

var car: Car

var previus_spring_length: float
var power_timer := 0.0

func _ready():
	car = get_parent().get_parent().car
	add_exception(car)
	previus_spring_length = car.suspension_rest_distance / 3
	
	
	if not use_as_steering:
		$debug_mesh.hide()
 

func _process(delta: float):
	if car.axel_input != 0:
		power_timer += delta
		print(power_timer, " inc?")
		if power_timer >= 10:
			power_timer = 10
		
	else:
		power_timer = 0
	
	print(power_timer, " axis: " , car.axel_input)
	
	# wheel rotation
	if use_as_steering != STEERING_TYPE.NONE:
		$debug_mesh.visible = car.debug
		
		if car.steering_input != 0:
			rotation.y = lerpf(rotation.y, car.steering_angle, car.steering_speed * delta)
		else:
			rotation.y = lerpf(rotation.y, car.steering_angle, car.steering_speed * 2 * delta)
	
	process_wheel_visuals(delta)


func process_wheel_visuals(delta: float):
	if $mesh == null:
		return
	
	var collision_point := get_collision_point()
	
	$mesh.global_position.y = collision_point.y + car.wheel_radius
	
	var speed = car.get_car_speed_ms() / (2 * PI * car.wheel_radius)
	var wheel_rotation_angle = speed * 2 * PI * delta
	$mesh.rotate_x(-wheel_rotation_angle)


func _physics_process(delta: float):
	if is_colliding():
		var collision_point := get_collision_point()
		
		suspension(delta, collision_point)
		steering_friction(delta, collision_point)
		acceleration(collision_point)
		friction(collision_point)
		#lateral_force(delta, collision_point)


func suspension(delta: float, raycast_dest: Vector3):
		var susp_direction := global_basis.y
		
		var raycast_origin := global_position
		var distance := raycast_dest.distance_to(raycast_origin)

		var spring_length := clampf(distance - car.wheel_radius, 0.2, car.suspension_rest_distance)
		var spring_force := car.spring_strength * (car.suspension_rest_distance - spring_length)
		var spring_velocity := (previus_spring_length - spring_length) / delta
		
		var damper_force := car.spring_damper * spring_velocity
		var suspension_force := basis.y * (spring_force + damper_force)
		
		previus_spring_length = spring_length 
		
		var point := get_wheel_point(raycast_dest)
		
		car.apply_force(susp_direction * suspension_force, point - car.global_position)
		
		if car.debug:
			DebugDraw3D.draw_line(point, point + Vector3(0, spring_length, 0), Color.BLACK, 0)
			DebugDraw3D.draw_arrow(global_position, to_global(position + Vector3(-position.x, (spring_velocity), -position.z)), Color.PINK, 0.1, true)
			DebugDraw3D.draw_arrow(global_position, to_global(position + Vector3(-position.x, (suspension_force.y / (2 * car.mass)), -position.z)), Color.YELLOW, 0.1, true)


func acceleration(collision_point: Vector3):
	if not use_as_traction:
		return
	
	var direction := -global_basis.z
	
	var torque := car.axel_input * (car.engine_power * power_timer / 10)
	var point := get_wheel_point(collision_point)
	
	car.apply_force(direction * torque, point - car.global_position)
	
	if car.debug:
		DebugDraw3D.draw_arrow(point, point + (direction * torque) / car.mass, Color.BLUE_VIOLET, 0.1, true)


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
		DebugDraw3D.draw_arrow(point, point + (-direction * force) / car.mass, Color.SANDY_BROWN, 0.1, true)


func get_point_velocity(point: Vector3) -> Vector3:
	return car.linear_velocity + car.angular_velocity.cross(point - car.global_position)


func steering_friction(delta: float, collision_point: Vector3):	
	var direction := global_basis.x
	var tire_word_vel := get_point_velocity(global_position)
	var steering_vel := direction.dot(tire_word_vel) 
	var acceleration = steering_vel / delta
	
	if abs(steering_vel) < 0.05:
		return
	
	var force = acceleration * car.mass / 80
	
	car.apply_force(-direction * force, collision_point - car.global_position)
	
	if car.debug:
		var point = get_wheel_point(collision_point)
		DebugDraw3D.draw_arrow(point, point + (direction * force) / car.mass, Color.BLACK, 0.1, true)


func get_distance_to_ground() -> float:
	return get_collision_point().distance_to(global_position)
