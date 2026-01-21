# Horizon
A simple terminal-based UI for ROS2 development, inspired by [Lazygit](https://github.com/jesseduffield/lazygit)

### Why Horizon
Robotic development can be complicated. Multiple terminal windows, several synchronous processes, etc. can cause confusion. Tools such as `rqt` already exist, but `Horizon` makes improvements:

- **Fast + Lightweight**: Built with Rust with zero overhead
- **No GUI needed**: Perfect over ssh on headless robots
- **Live Monitoring**: Real-time frequency + data throughput for topics
- **Quick Actions**: Restart nodes, echo topics, or edit parameters with one key

### Installation 

```bash
cargo build --release
```

### Usage

Run Horizon from your ROS2 workspace root:

```bash
cd ~/your_ros2_workspace
/path/to/Horizon/target/release/Horizon
```

Or use the test script:
```bash
./test_Horizon.sh ~/your_ros2_workspace
```

**Important**: Make sure ROS2 is sourced in your environment:
```bash
source /opt/ros/humble/setup.bash  # or your ROS2 distro
```

### Cheat Sheet

| Key | Action |
|-----|--------|
| b | Build and install package (colcon build --symlink-install) |
| q | Quit |

### Testing

See [TESTING.md](TESTING.md) for detailed testing instructions with real ROS2 packages.
