import pytest
from deuterium import Matrix4, Vector3


def test_constructor():
    assert Matrix4.identity()


# def test_repr():
#     v = Vector3(1.0, 2.0, 3.0)
#     assert repr(v) == "Vector3(1, 2, 3)"
#     assert repr(v) == str(v)


def test_indexing():
    m = Matrix4.identity()
    assert m[0] == (1, 0, 0, 0)
    assert m[1] == (0, 1, 0, 0)
    assert m[2] == (0, 0, 1, 0)
    assert m[3] == (0, 0, 0, 1)

    assert m[0, 0] == 1
    assert m[0, 1] == 0
    assert m[1, 1] == 1
    assert m[1, 2] == 0
    assert m[2, 2] == 1

    with pytest.raises(IndexError):
        m[4]

    with pytest.raises(IndexError):
        m[-5]

    with pytest.raises(IndexError):
        m[1, 4]

    with pytest.raises(IndexError):
        m[4, 1]

    with pytest.raises(IndexError):
        m[1, -1]


    with pytest.raises(IndexError):
        m[-1, 1]


def test_index_mutation():
    m = Matrix4.identity()
    for a in range(4):
        for b in range(4):
            m[a, b] = a * 10 + b

    for a in range(4):
        for b in range(4):
            assert m[a, b] == a * 10 + b


def test_translation():
    m = Matrix4.identity()
    m.translate(Vector3(1, 2, 3))
    assert m.translation == Vector3(1, 2, 3)
    m.translation = Vector3(3, 2, 1)
    assert m.translation == Vector3(3, 2, 1)
    assert m == Matrix4.from_translation(Vector3(3, 2, 1))


def test_transpose():
    m = Matrix4.identity()
    m[0, 1] = 5
    m[1, 0] = -5
    mt = m.transposed()
    assert mt[0, 1] == -5
    assert mt[1, 0] == 5


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
