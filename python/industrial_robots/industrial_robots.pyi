from __future__ import annotations

from typing import Tuple

from numpy.typing import NDArray
from numpy import uint32


def micro_serialize(vertices: NDArray[float], faces: NDArray[uint32]) -> bytes:
    """

    :param vertices:
    :param faces:
    :return:
    """
    ...


def micro_deserialize(data: bytes) -> Tuple[NDArray[float], NDArray[uint32]]:
    """
    Deserialize a byte array into a tuple of vertices and faces.

    :param data: the byte array to deserialize
    :return: a tuple of vertices and faces
    """
    ...


class Frame3:
    """

    """

    def __init__(self, matrix: NDArray[float]):
        """
        Attempt to create a frame from a 4x4 matrix in the form of a numpy array. If the matrix is not a valid
        isometry (it is not orthogonal, it has scale or shear, etc.), an exception will be raised.

        Use this method if you explicitly have a known matrix to convert to an isometry, otherwise consider using a
        composition of the `from_translation` and `from_rotation` methods.

        :param matrix: a numpy array of shape (4, 4) containing the matrix representation of the isometry.
        """
        ...

    @staticmethod
    def identity() -> Frame3:
        """ Return the identity (origin) frame. """
        ...

    @staticmethod
    def from_translation(x: float, y: float, z: float) -> Frame3:
        """
        Create a frame representing a translation by the specified x, y, and z components.
        :param x: the x component of the translation.
        :param y: the y component of the translation.
        :param z: the z component of the translation.
        :return: an isometry containing only a translation component
        """
        ...

    @staticmethod
    def from_rotation(angle: float, ax: float, ay: float, az: float) -> Frame3:
        """
        Create a frame representing a rotation around an axis defined by a vector direction and the origin. The
        components of the direction will be automatically normalized before the rotation applied.

        When looking down the axis of rotation (the axis is pointing towards the observer), the rotation will be
        counter-clockwise.

        :param angle: the angle to rotate by in radians.
        :param ax: the x component of the rotation axis.
        :param ay: the y component of the rotation axis.
        :param az: the z component of the rotation axis.
        :return: the isometry representing the rotation.
        """
        ...

    def __matmul__(self, other: Frame3) -> Frame3:
        """
        Multiply another frame by the frame, transforming it and returning a new frame
        :param other: the frame to transform
        :return: a new frame that is the result of the transformation
        """
        ...

    def inverse(self) -> Frame3:
        """
        Get the inverse of the frame. The inverse is the transform that will undo the transformation of the original
        frame, or the frame that when applied to the original frame will return the identity (origin) frame.
        """
        ...

    def transform_points(self, points: NDArray[float]) -> NDArray[float]:
        """
        Transform an array of points into the frame. The semantics of transforming points are such that the full
        matrix is applied, first rotating the point around the origin and then translating it by the translation vector.

        To transform vectors, use the `transform_vectors` method instead.

        This is an efficient way to transform a large number of points at once, rather than using the `@` operator
        individually on a large number of `Point3` objects.

        :param points: a numpy array of shape (N, 3)
        :return: a numpy array of shape (N, 3) containing the transformed points in the same order as the input.
        """
        ...

    def transform_vectors(self, vectors: NDArray[float]) -> NDArray[float]:
        """
        Transform an array of vectors into the frame. The semantics of transforming vectors are such that only the
        rotation matrix is applied, and the translation vector is not used. The vectors retain their original
        magnitude, but their direction is rotated by the isometry.

        To transform points, use the `transform_points` method instead.

        This is an efficient way to transform a large number of vectors at once, rather than using the `@` operator
        individually on a large number of `Vector3` objects.

        :param vectors: a numpy array of shape (N, 3)
        :return: a numpy array of shape (N, 3) containing the transformed vectors in the same order as the input.
        """
        ...

    def as_numpy(self) -> NDArray[float]:
        """
        Return a copy of the 4x4 matrix representation of the isometry.
        """
        ...
