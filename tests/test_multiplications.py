from deuterium import *


def test_mat4_vec3():
    m = Matrix4.identity()
    v = Vector3(1, 2, 3)
    assert m * v == Vector3(1, 2, 3)


def test_mat4_vec3():
    m = Matrix4.identity()
    v = Vector3(1, 2, 3)
    assert m * v == Vector3(1, 2, 3)


def test_iso_vec3():
    i = Isometry3.identity()
    v = Vector3(1, 2, 3)
    assert i * v == Vector3(1, 2, 3)


def test_quat_vec3():
    q = UnitQuaternion.identity()
    v = Vector3(1, 2, 3)
    assert q * v == Vector3(1, 2, 3)

