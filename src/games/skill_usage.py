import pyscreenshot as ImageGrab
import cv2
import numpy as np
import json
import sys

def capture_screenshot():
    # Capture a full-screen screenshot
    screenshot = ImageGrab.grab()
    # Convert the screenshot to a NumPy array
    screenshot_np = np.array(screenshot)
    # Convert the screenshot to BGR (OpenCV format)
    screenshot_bgr = cv2.cvtColor(screenshot_np, cv2.COLOR_RGB2BGR)
    return screenshot_bgr

def find_image_on_screen(template_path):
    # Capture the screenshot
    screen_img = capture_screenshot()
    
    # Read the template image
    template = cv2.imread(template_path, cv2.IMREAD_COLOR)
    if template is None:
        print(json.dumps({"status": "error", "message": "Template image not found"}))
        return None
    
    # Perform template matching
    result = cv2.matchTemplate(screen_img, template, cv2.TM_CCOEFF_NORMED)
    
    # Get the best match position
    min_val, max_val, min_loc, max_loc = cv2.minMaxLoc(result)
    
    # Set a threshold for the match quality
    threshold = 0.8
    if max_val >= threshold:
        return max_loc
    else:
        return None

def main():
    if len(sys.argv) != 2:
        print(json.dumps({"status": "error", "message": "Invalid arguments"}))
        return

    template_path = sys.argv[1]
    match_location = find_image_on_screen(template_path)
    
    if match_location:
        print(json.dumps({"status": "success", "position": match_location}))
    else:
        print(json.dumps({"status": "error", "message": "Image not found"}))

if __name__ == "__main__":
    main()
