extends Node2D
class_name Speedometer

func set_gear(gear: int):
	$Gear.text = "Gear: " + str(gear)


func set_speed(speed: int):
	$Speed.text = "Speed: " + str(speed) + " km\\h"


func set_rpm(rpm: int):
	$Rpm.text = "Rpm: " + str(rpm)
