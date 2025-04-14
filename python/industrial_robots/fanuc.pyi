from __future__ import annotations

from typing import List, Tuple
from numpy.typing import NDArray
from numpy import uint32

from .industrial_robots import Frame3


class Crx:
    """
    Class representing a FANUC CRX robot.
    """

    @staticmethod
    def new_5ia() -> Crx:
        """
        Create a new FANUC CRX 5ia robot.
        :return: a new instance of the FANUC CRX 5ia robot
        """
        ...

    @staticmethod
    def new_10ia() -> Crx:
        """
        Create a new FANUC CRX 10ia robot.
        :return: a new instance of the FANUC CRX 10ia robot
        """
        ...

    @staticmethod
    def fk(joints: List[float]) -> Frame3:
        """
        Compute the forward kinematics of the FANUC CRX robot.
        :param joints: a list of 6 FANUC joint angles in degrees, the way they would be entered into the controller
        :return: a Frame3 object representing the end-effector pose
        """
        ...

    def get_meshes(self) -> List[Tuple[NDArray[float], NDArray[uint32]]]:
        """
        Get the meshes of the FANUC CRX robot.
        :return: a list of tuples, each containing a numpy array of vertices and a numpy array of faces
        """
        ...

    def fk_all(self, joints: List[float]) -> List[Frame3]:
        """
        Compute the forward kinematics of the FANUC CRX robot with all links.
        :param joints: a list of 6 FANUC joint angles in degrees, the way they would be entered into the controller
        :return: a list of Frame3 objects representing the pose of each link, including the end-effector
        """
        ...