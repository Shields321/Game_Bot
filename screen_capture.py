import mss
import cv2
import numpy as np
import sys
import json
import pytesseract
import time

def capture_screen_text():
    with mss.mss() as sct:
        # Capture the entire screen
        monitor = sct.monitors[1]
        screenshot = sct.grab(monitor)
        img = np.array(screenshot)

        # Convert the image to grayscale
        gray_img = cv2.cvtColor(img, cv2.COLOR_BGRA2GRAY)

        # Perform OCR to detect words
        text = pytesseract.image_to_string(gray_img)

        # Send the data as JSON to the Rust process through standard output
        data = {"detected_text": text}
        sys.stdout.write(json.dumps(data) + "\n")
        sys.stdout.flush()

def capture_screen_pixels():
    with mss.mss() as sct:
        # Capture the entire screen
        monitor = sct.monitors[1]
        screenshot = sct.grab(monitor)
        img = np.array(screenshot)
        # Convert the image to RGB
        rgb_img = cv2.cvtColor(img, cv2.COLOR_BGRA2RGB)
        return rgb_img

def json_default(value):
    """Convert numpy types to Python types for JSON serialization."""
    if isinstance(value, np.bool_):
        return bool(value)
    raise TypeError(f"Object of type {type(value)} is not JSON serializable")

def detect_movement(prev_img, curr_img, threshold=30):
    # Compute the absolute difference between the current frame and the previous frame
    diff = cv2.absdiff(prev_img, curr_img)
    # Convert the difference image to grayscale
    gray_diff = cv2.cvtColor(diff, cv2.COLOR_BGR2GRAY)
    # Apply a binary threshold to the grayscale difference image
    _, thresh = cv2.threshold(gray_diff, 50, 255, cv2.THRESH_BINARY)
    # Calculate the percentage of the image that has changed
    change_percentage = (np.sum(thresh) / 255) / (thresh.shape[0] * thresh.shape[1])
    # Determine if the change percentage exceeds the threshold
    movement_detected = change_percentage > (threshold / 100.0)    
    return movement_detected

if __name__ == "__main__":
    prev_img = None
    while True:
        curr_img = capture_screen_pixels()
        if prev_img is not None:
            movement_detected = detect_movement(prev_img, curr_img)
            data = {"movement_detected": movement_detected}
            sys.stdout.write(json.dumps(data, default=json_default) + "\n")
            sys.stdout.flush()
        prev_img = curr_img
        time.sleep(2)  # Simulate a delay between captures
