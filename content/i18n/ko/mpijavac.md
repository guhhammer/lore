# mpijavac

Open MPI용 Java 컴파일러 래퍼.

## Usage
- `mpijavac 경로/대상/소스_파일.java` : java 소스 파일 컴파일
- `mpijavac -cp 경로/대상/애플리케이션.jar 경로/대상/소스_파일.java` : 애플리케이션 전용 클래스패스를 전달하여 컴파일
- `mpijavac --showme` : MPI Java 애플리케이션 빌드에 필요한 플래그 표시
- `mpijavac --showme:compile` : MPI Java 애플리케이션 컴파일에 필요한 플래그만 표시
- `mpijavac 경로/대상/소스_파일.java --showme` : 실제로 호출되는 Java 컴파일러 명령 전체 표시
