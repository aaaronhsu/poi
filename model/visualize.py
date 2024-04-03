from ultralytics import YOLO

# ------ CONFIGURATION ------
video_name: str = 'antispin'
weights_path: str = 'runs/detect/train7/weights/best.pt'


model = YOLO(weights_path)
results = model.track(source=f'./dataset/videos/{video_name}.mp4', show=True, tracker='bytetrack.yaml')