# gdal_contour

디지털 표고 모델에서 등고선과 다각형을 생성.

## Usage
- `gdal_contour -a ele -i 100.0 경로/대상/입력.tif 경로/대상/출력.gpkg` : 고도 속성(attributing)을 "ele"로 지정하면서 100미터 간격(interval)에 걸쳐 등고선이 퍼져있는 벡터 데이터세트를 생성
- `gdal_contour -i 100.0 -p 경로/대상/입력.tif 경로/대상/출력.gpkg` : 100미터 간격(interval)에 걸쳐 분산된 다각형(polygons)으로 벡터 데이터세트를 생성

## Links
- More information: https://gdal.org/en/stable/programs/gdal_contour.html
