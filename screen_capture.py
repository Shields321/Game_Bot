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

        # Convert the image to grayscale
        gray_img = cv2.cvtColor(img, cv2.COLOR_BGRA2GRAY)

        # Perform OCR to detect words
        text = pytesseract.image_to_string(gray_img)

        # Send the data as JSON to the Rust process through standard output
        data = {"detected_text": text}
        sys.stdout.write(json.dumps(data) + "\n")
        sys.stdout.flush()

if __name__ == "__main__":
    while True:
        capture_screen_text()
        time.sleep(10)  # Simulate a delay between captures
