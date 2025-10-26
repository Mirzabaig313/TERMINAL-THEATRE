"""Save/Load system for Terminal Theatre"""

import json
import os
import time
from pathlib import Path
from typing import Dict, List, Optional, Any
from datetime import datetime
from dataclasses import dataclass, asdict


@dataclass
class SaveMetadata:
    """Metadata for a save file"""
    slot: int
    save_name: str
    timestamp: str
    game_version: str
    current_scene: str
    scene_description: str
    playtime: float
    completion_percentage: float
    visited_scenes_count: int
    total_choices_made: int
    
    def to_dict(self) -> Dict:
        """Convert to dictionary"""
        return asdict(self)
    
    @staticmethod
    def from_dict(data: Dict) -> 'SaveMetadata':
        """Create from dictionary"""
        return SaveMetadata(**data)


class SaveManager:
    """Manages game save/load operations"""
    
    DEFAULT_SAVE_DIR = Path.home() / ".terminal_theatre" / "saves"
    AUTOSAVE_SLOT = 0
    MAX_SAVE_SLOTS = 10
    GAME_VERSION = "1.0.0"
    
    def __init__(self, save_dir: Optional[Path] = None):
        """Initialize save manager"""
        self.save_dir = Path(save_dir) if save_dir else self.DEFAULT_SAVE_DIR
        self._ensure_save_directory()
    
    def _ensure_save_directory(self):
        """Create save directory if it doesn't exist"""
        self.save_dir.mkdir(parents=True, exist_ok=True)
    
    def _get_save_path(self, slot: int) -> Path:
        """Get path for a save slot"""
        if slot == self.AUTOSAVE_SLOT:
            return self.save_dir / "autosave.json"
        return self.save_dir / f"save_slot_{slot}.json"
    
    def save_game(self, slot: int, game_state: Dict[str, Any], 
                  save_name: Optional[str] = None) -> bool:
        """
        Save game state to a slot
        
        Args:
            slot: Save slot number (0 for autosave)
            game_state: Dictionary containing game state
            save_name: Optional custom name for the save
        
        Returns:
            True if save successful, False otherwise
        """
        try:
            # Generate save name if not provided
            if save_name is None:
                if slot == self.AUTOSAVE_SLOT:
                    save_name = "Autosave"
                else:
                    save_name = f"Save {slot}"
            
            # Create metadata
            current_scene = game_state.get('current_scene', 'unknown')
            scene_description = self._get_scene_description(
                current_scene, 
                game_state.get('story_title', 'Unknown Story')
            )
            
            metadata = SaveMetadata(
                slot=slot,
                save_name=save_name,
                timestamp=datetime.now().isoformat(),
                game_version=self.GAME_VERSION,
                current_scene=current_scene,
                scene_description=scene_description,
                playtime=game_state.get('playtime', 0.0),
                completion_percentage=self._calculate_completion(game_state),
                visited_scenes_count=len(game_state.get('visited_scenes', [])),
                total_choices_made=len(game_state.get('choice_history', []))
            )
            
            # Prepare save data
            save_data = {
                'metadata': metadata.to_dict(),
                'game_state': game_state
            }
            
            # Write to file
            save_path = self._get_save_path(slot)
            with open(save_path, 'w', encoding='utf-8') as f:
                json.dump(save_data, f, indent=2, ensure_ascii=False)
            
            return True
            
        except Exception as e:
            print(f"Error saving game: {e}")
            return False
    
    def load_game(self, slot: int) -> Optional[Dict[str, Any]]:
        """
        Load game state from a slot
        
        Args:
            slot: Save slot number to load from
        
        Returns:
            Game state dictionary if successful, None otherwise
        """
        try:
            save_path = self._get_save_path(slot)
            
            if not save_path.exists():
                return None
            
            with open(save_path, 'r', encoding='utf-8') as f:
                save_data = json.load(f)
            
            # Validate save data
            if not self._validate_save_data(save_data):
                print(f"Warning: Save file in slot {slot} appears corrupted")
                return None
            
            # Check version compatibility
            metadata = SaveMetadata.from_dict(save_data['metadata'])
            if not self._is_compatible_version(metadata.game_version):
                print(f"Warning: Save file was created with version {metadata.game_version}")
                print(f"Current version is {self.GAME_VERSION}. Some features may not work correctly.")
            
            return save_data['game_state']
            
        except json.JSONDecodeError:
            print(f"Error: Save file in slot {slot} is corrupted")
            return None
        except Exception as e:
            print(f"Error loading game: {e}")
            return None
    
    def get_save_metadata(self, slot: int) -> Optional[SaveMetadata]:
        """Get metadata for a save slot without loading the full save"""
        try:
            save_path = self._get_save_path(slot)
            
            if not save_path.exists():
                return None
            
            with open(save_path, 'r', encoding='utf-8') as f:
                save_data = json.load(f)
            
            return SaveMetadata.from_dict(save_data['metadata'])
            
        except Exception:
            return None
    
    def list_saves(self) -> List[Optional[SaveMetadata]]:
        """List all available saves"""
        saves = []
        for slot in range(self.MAX_SAVE_SLOTS):
            metadata = self.get_save_metadata(slot)
            saves.append(metadata)
        return saves
    
    def delete_save(self, slot: int) -> bool:
        """Delete a save slot"""
        try:
            save_path = self._get_save_path(slot)
            if save_path.exists():
                save_path.unlink()
                return True
            return False
        except Exception as e:
            print(f"Error deleting save: {e}")
            return False
    
    def get_last_save_slot(self) -> Optional[int]:
        """Get the most recent save slot (excluding autosave)"""
        latest_slot = None
        latest_time = None
        
        for slot in range(1, self.MAX_SAVE_SLOTS):
            metadata = self.get_save_metadata(slot)
            if metadata:
                timestamp = datetime.fromisoformat(metadata.timestamp)
                if latest_time is None or timestamp > latest_time:
                    latest_time = timestamp
                    latest_slot = slot
        
        return latest_slot
    
    def has_autosave(self) -> bool:
        """Check if an autosave exists"""
        return self._get_save_path(self.AUTOSAVE_SLOT).exists()
    
    def _validate_save_data(self, save_data: Dict) -> bool:
        """Validate save data structure"""
        required_keys = ['metadata', 'game_state']
        if not all(key in save_data for key in required_keys):
            return False
        
        required_metadata = ['slot', 'timestamp', 'game_version', 'current_scene']
        if not all(key in save_data['metadata'] for key in required_metadata):
            return False
        
        required_game_state = ['current_scene', 'visited_scenes', 'flags']
        if not all(key in save_data['game_state'] for key in required_game_state):
            return False
        
        return True
    
    def _is_compatible_version(self, saved_version: str) -> bool:
        """Check if saved version is compatible with current version"""
        # For now, just check major version
        try:
            saved_major = int(saved_version.split('.')[0])
            current_major = int(self.GAME_VERSION.split('.')[0])
            return saved_major == current_major
        except (ValueError, IndexError):
            return False
    
    def _get_scene_description(self, scene_id: str, story_title: str) -> str:
        """Generate a human-readable scene description"""
        # Format scene ID for display
        scene_name = scene_id.replace('_', ' ').title()
        return f"{story_title} - {scene_name}"
    
    def _calculate_completion(self, game_state: Dict) -> float:
        """Calculate completion percentage"""
        visited = len(game_state.get('visited_scenes', []))
        total = game_state.get('total_scenes', 1)
        
        if total == 0:
            return 0.0
        
        return min(100.0, (visited / total) * 100.0)
    
    def export_save(self, slot: int, export_path: Path) -> bool:
        """Export a save to a specific path (for backup/sharing)"""
        try:
            save_path = self._get_save_path(slot)
            if not save_path.exists():
                return False
            
            import shutil
            shutil.copy2(save_path, export_path)
            return True
        except Exception as e:
            print(f"Error exporting save: {e}")
            return False
    
    def import_save(self, import_path: Path, slot: int) -> bool:
        """Import a save from a specific path"""
        try:
            if not import_path.exists():
                return False
            
            # Validate the import file
            with open(import_path, 'r', encoding='utf-8') as f:
                save_data = json.load(f)
            
            if not self._validate_save_data(save_data):
                print("Error: Invalid save file format")
                return False
            
            # Copy to save directory
            save_path = self._get_save_path(slot)
            import shutil
            shutil.copy2(import_path, save_path)
            return True
            
        except Exception as e:
            print(f"Error importing save: {e}")
            return False
