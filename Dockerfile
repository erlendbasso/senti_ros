FROM  arm64v8/ros:rolling

# ENV ROS_DISTRO=humble
ENV ROS_DISTRO=rolling

# Change the default shell to Bash
SHELL [ "/bin/bash" , "-c" ]

RUN echo "source /opt/ros/${ROS_DISTRO}/setup.bash && echo Sourced ROS-${ROS_DISTRO}" >> /root/.bashrc;

# RUN apt-get update && apt-get install 

# INSTALL OPENCV
RUN apt-get update && apt-get install -y cmake g++ wget unzip git python3-colcon-common-extensions llvm clang

RUN apt-get install -y curl

# # Create a Catkin workspace and clone TurtleBot3 repos
# RUN source /opt/ros/${ROS_DISTRO}/setup.bash \
#  && mkdir -p /workspaces/ros2_ws/src \
#  && cd /workspaces/ros2_ws/src \
#  && git clone https://turlab.itk.ntnu.no/turlab/bluerov_interfaces.git
# # RUN echo "export TURTLEBOT3_MODEL=waffle_pi" >> ~/.bashrc
 
# # Build the Catkin workspace and ensure it's sourced
# RUN source /opt/ros/${ROS_DISTRO}/setup.bash \
#  && cd /workspaces/ros2_ws \
#  && colcon build
# RUN echo "source /workspaces/ros2_ws/install/setup.bash" >> ~/.bashrc
 
# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# Set the working folder at startup
# WORKDIR /workspaces/ros2_ws


# CMD ["/bin/bash"]