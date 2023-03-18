import pytest
from math import radians
from deuterium import Vector3, UnitQuaternion


def test_constructor():
    assert UnitQuaternion()
    assert UnitQuaternion() == UnitQuaternion.identity()


def test_euler_convention():
    roll, pitch, yaw = radians(30), radians(45), radians(60)

    quat_roll = UnitQuaternion.from_axis_angle(Vector3(1, 0, 0), roll)
    quat_pitch = UnitQuaternion.from_axis_angle(Vector3(0, 1, 0), pitch)
    quat_yaw = UnitQuaternion.from_axis_angle(Vector3(0, 0, 1), yaw)

    quat_multiplied = quat_yaw * quat_pitch * quat_roll

    quat_constructed = UnitQuaternion.from_euler(roll, pitch, yaw)

    assert quat_multiplied.approx_equals(quat_constructed)
    
    quat_yaw = UnitQuaternion.from_axis_angle(Vector3(0, 0, 1), radians(30))

    assert quat_yaw.axis() == Vector3(0, 0, 1)
    assert quat_yaw.angle() == radians(30)

    assert quat_yaw.euler() == (0, 0, radians(30))


def test_slerp():
    quat1 = UnitQuaternion.from_axis_angle(Vector3(1, 0, 0), radians(30))
    quat2 = UnitQuaternion.from_axis_angle(Vector3(1, 0, 0), radians(60))

    slerp_0 = quat1.slerp(quat2, 0)
    slerp_1 = quat1.slerp(quat2, 1)
    slerp_half = quat1.slerp(quat2, 0.5)

    assert slerp_0.approx_equals(quat1)
    assert slerp_1.approx_equals(quat2)
    
    # Check if slerp(0.5) is close to the quaternion from_axis_angle with half the angle
    quat_half_angle = UnitQuaternion.from_axis_angle(Vector3(1, 0, 0), radians(45))
    assert slerp_half.approx_equals(quat_half_angle)
