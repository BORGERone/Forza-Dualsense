"""IPC Server for Tauri - communicates with existing DualSense logic."""
import sys
import json
import logging
from pathlib import Path

# Setup logging to stderr only
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    stream=sys.stderr
)
log = logging.getLogger("fhds.ipc")

log.info("=" * 50)
log.info("Starting IPC Server")
log.info(f"Python version: {sys.version}")
log.info(f"Working directory: {Path.cwd()}")
log.info(f"Script location: {Path(__file__)}")

# Add the backend directory to path to import modules
backend_dir = Path(__file__).parent
sys.path.insert(0, str(backend_dir))
log.info(f"Backend directory: {backend_dir}")

try:
    log.info("Attempting to import modules...")
    from modules import dualsense, udplistener, loop
    log.info("Successfully imported dualsense, udplistener, loop")
    
    from modules.settings import Settings
    log.info("Successfully imported Settings")
    
    from modules import preferences
    log.info("Successfully imported preferences")
    
except ImportError as e:
    error_msg = f"Import error: {str(e)}"
    log.error(error_msg)
    print(json.dumps({"error": error_msg}), flush=True)
    sys.exit(1)
except Exception as e:
    error_msg = f"Initialization error: {str(e)}"
    log.error(error_msg)
    print(json.dumps({"error": error_msg}), flush=True)
    sys.exit(1)

log.info("All imports successful")


class IPCServer:
    """IPC server that communicates with Tauri frontend via stdin/stdout."""
    
    def __init__(self):
        self.settings = Settings()
        preferences.load(self.settings)
        self.ds = None
        self.listener = None
        self.running = False
        self._loop_thread = None
        
    def start(self):
        """Start the DualSense and UDP listener."""
        log.info("Starting IPC server")
        self.running = True
        
        # Initialize DualSense
        self.ds = dualsense.DualSense(
            startup_pulse_force=self.settings.startup_pulse_force,
            enable_startup_pulse=self.settings.enable_startup_pulse,
            reconnect_interval_s=self.settings.reconnect_interval_s,
        )
        self.ds.open()
        
        # Start UDP listener in background
        self.listener = udplistener.UDPListener(
            self.settings.udp_host, 
            self.settings.udp_port, 
            self.settings.udp_timeout
        )
        self.listener.__enter__()
        log.info(f"Listening on {self.settings.udp_host}:{self.settings.udp_port}")
        
        # Start main loop in background thread
        import threading
        self._loop_thread = threading.Thread(target=self._run_loop, daemon=True)
        self._loop_thread.start()
        
        log.info("IPC server started")
        self._send_response({"status": "started", "message": "Backend started successfully"})
    
    def _run_loop(self):
        """Run the main telemetry processing loop."""
        try:
            loop.run(self.ds, self.listener, self.settings)
        except Exception as e:
            log.error(f"Loop error: {e}")
            self._send_response({"error": f"Loop error: {str(e)}"})
        
    def stop(self):
        """Stop the DualSense and UDP listener."""
        log.info("Stopping IPC server")
        self.running = False
        
        # Stop loop thread
        if self._loop_thread and self._loop_thread.is_alive():
            # The loop will exit when self.running is False
            self._loop_thread.join(timeout=2.0)
        
        if self.listener:
            try:
                self.listener.__exit__(None, None, None)
            except Exception as e:
                log.warning(f"Error closing UDP listener: {e}")
            self.listener = None
            
        if self.ds:
            try:
                self.ds.close()
            except Exception as e:
                log.warning(f"Error closing DualSense: {e}")
            self.ds = None
            
        try:
            self._send_response({"status": "stopped", "message": "Backend stopped successfully"})
        except OSError:
            # Ignore errors when stdout is already closed
            pass
        
    def handle_command(self, command: dict):
        """Handle incoming command from Tauri frontend."""
        cmd_type = command.get("type")
        
        if cmd_type == "start":
            self.start()
        elif cmd_type == "stop":
            self.stop()
        elif cmd_type == "get_status":
            self._send_status()
        elif cmd_type == "update_settings":
            self._update_settings(command.get("settings", {}))
        else:
            self._send_response({"error": f"Unknown command: {cmd_type}"})
            
    def _send_response(self, data: dict):
        """Send response to Tauri frontend via stdout."""
        try:
            print(json.dumps(data), flush=True)
        except OSError:
            # Ignore errors when stdout is closed
            pass
        
    def _send_status(self):
        """Send current status to Tauri frontend."""
        status = {
            "type": "status",
            "dualsense_connected": self.ds.connected if self.ds else False,
            "running": self.running,
            "settings": {
                "udp_host": self.settings.udp_host,
                "udp_port": self.settings.udp_port,
                "startup_pulse_force": self.settings.startup_pulse_force,
                "enable_startup_pulse": self.settings.enable_startup_pulse,
            }
        }
        self._send_response(status)
    
    def _update_settings(self, new_settings: dict):
        """Update settings from Tauri frontend."""
        try:
            for key, value in new_settings.items():
                if hasattr(self.settings, key):
                    setattr(self.settings, key, value)
                    log.info(f"Updated setting {key} = {value}")
            
            # Save to preferences file
            preferences.save(self.settings)
            self._send_response({"status": "success", "message": "Settings updated successfully"})
        except Exception as e:
            log.error(f"Error updating settings: {e}")
            self._send_response({"error": f"Failed to update settings: {str(e)}"})
            
    def run(self):
        """Main loop - read commands from stdin and handle them."""
        log.info("IPC server ready for commands")
        
        try:
            for line in sys.stdin:
                try:
                    command = json.loads(line.strip())
                    self.handle_command(command)
                except json.JSONDecodeError:
                    log.error(f"Invalid JSON: {line}")
                    self._send_response({"error": "Invalid JSON"})
                except Exception as e:
                    log.error(f"Error handling command: {e}")
                    self._send_response({"error": str(e)})
                    
        except KeyboardInterrupt:
            log.info("IPC server interrupted")
        finally:
            self.stop()


if __name__ == "__main__":
    server = IPCServer()
    server.run()
