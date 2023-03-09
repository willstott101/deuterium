import pytest
from deuterium import Isometry3, Matrix4, Vector3


def test_constructor():
    assert Matrix4.identity()


# def test_repr():
#     v = Vector3(1.0, 2.0, 3.0)
#     assert repr(v) == "Vector3(1, 2, 3)"
#     assert repr(v) == str(v)


def test_translation():
    iso = Isometry3.identity()
    iso.translate(Vector3(1, 2, 3))
    assert iso.translation == Vector3(1, 2, 3)
    iso.translation = Vector3(3, 2, 1)
    assert iso.translation == Vector3(3, 2, 1)
    assert iso == Isometry3.from_translation(Vector3(3, 2, 1))

# def test_ops():
#     assert Vector3(1, 2, 3) + Vector3(10, 20, 30) == Vector3(11, 22, 33)
#     assert Vector3(1, 2, 3) - Vector3(10, 20, 30) == Vector3(-9, -18, -27)
#     v = Vector3(1, 2, 3)
#     v += Vector3(10, 20, 30)
#     assert v == Vector3(11, 22, 33)
#     v -= Vector3(10, 20, 30)
#     assert v == Vector3(1, 2, 3)
#     assert v * 5 == Vector3(5, 10, 15)
#     v *= 10
#     assert v == Vector3(10, 20, 30)
#     assert -v == Vector3(-10, -20, -30)
#     v.negate()
#     assert v == Vector3(-10, -20, -30)
