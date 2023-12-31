### 1. 제한적 직접 실행 원리
- CPU 시간을 나누어 씀으로써 가상화를 구현할 수 있음
- 가상화 기법을 구현하기 위해 해결해야 할 것
  - 첫 번째는 성능 저하 문제 : 시스템에 과중한 오버헤드를 주지 않으면서 가상화를 구현할 수 있도록
  - 두 번째는 제어 문제 : CPU에 대한 통제를 유지하면서 프로세스를 효율적으로 실행시킬 수 있는 방법은 무엇인지
- 운영체제의 입장에서는 자원 관리의 책임자로서 특히 제어 문제가 중요
- 제어권을 상실하면 한 프로세스가 영원히 실행을 계속할 수 있고 컴퓨터를 장악하거나 접근해서는 안 되는 정보에 접근하게 됨

<br>

- 제어를 유지하면서 효과적으로 CPU를 가상화하는 방법
  - 운영체제는 효율적인 방식으로 CPU를 가상화하지만 시스템에 대한 제어를 잃지는 않음
  - 제어를 잃지 않기 위해서 하드웨어와 운영체제의 지원이 필수적
  - 종종 운영체제는 작업을 효과적으로 수행하기 위하여 하드웨어가 제공하는 기능을 신중하게 사용


<br>

#### 1.1. 기본 원리 : 제한적 직접 실행
- 운영체제 개발자들은 프로그램을 빠르게 실행하기 위하여 제한적 직접 실행(Limited
Direct Execution) 이라는 기법을 개발
- 프로그램을 CPU 상에서 그냥 직접 실행시키는 것
- 따라서 운영체제가 프로그램을 실행하기 시작할 때 프로세스 목록에 해당 프로세스 항목을 만들고 메모리를 할당하며 프로그램 코드를 디스크에서 탑재하고 진입점 (예, main() 루틴 혹은 유사한 무엇)을 찾아 그 지점으로 분기하여 사용자 코드를 실행하기 시작
- 직접 실행 프로토콜(제한 없음)
  ```
    운영체제                          프로그램
    ==============================================================
    프로세스 목록의 항목을 생성
    프로그램 메모리 할당
    메모리에 프로그램 탑재
    argc/argv를 위한 스택 셋ᨦ
    레지스터 내용 삭제
    call main() 실행
                                      main() 실행
                                      main에서 return 명령어 실행
    프로세스 메모리 반환
    프로세스 목록에서 항목 제거
  ``` 

- 그러나 이 접근법은 CPU를 가상화함에 있어 몇 가지 문제를 일으킴
  - 첫 번째는 프로그램을 직접 실행시킨다면 프로그램이 운영체제가 원치않는 일을 하지 않는다는 것을 어떻게 보장할 수 있는지
  - 두 번째는 프로세스 실행 시 운영체제는 어떻게 프로그램의 실행을 중단하고 다른 프로세스로 전환시킬 수 있는지
    - 즉, CPU를 가상화하는 데 필요한 시분할(time sharing) 기법을 어떻게 구현할 수 있는지


<br>

#### 1.2. 문제점 1 : 제한된 연산
- 직접 실행의 장점은 빠르게 실행된다는 것
- 기본적으로 프로그램이 하드웨어 CPU에서 실행되기 때문
- 그러나 CPU에서 직접 실행시키면 새로운 문제가 발생
  - 이 때문에 **사용자 모드(user mode)** 라고 알려진 새로운 모드가 도입
    - 사용자 모드에서 실행되는 코드는 할 수 있는 일이 제한됨
  - **커널 모드(kernel mode)** 는 사용자 모드와 대비되는 모드로서 운영체제의 중요한 코드들이 실행됨
    - 이 모드에서 실행되는 코드는 모든 특수한 명령어를 포함하여 원하는 모든 작업을 수행할 수 있음
  - 사용자 프로세스가 특권 명령어를 실행해야 할 경우
    - 거의 모든 현대 하드웨어는 사용자 프로세스에게 **시스템 콜**을 제공
    - 기능에는 파일 시스템 접근, 프로세스 생성 및 제거, 다른 프로세스와의 통신 및 메모리 할당 등이 포함
    - 시스템 콜을 실행하기 위해 프로그램은 trap 특수 명령어를 실행해야 함
    - 이 명령어는 커널 안으로 분기하는 동시에 특권 수준을 커널 모드로 상향 조정
    - 커널 모드로 진입하면 운영체제는 모든 명령어를 실행할 수 있고 이를 통하여 프로세스가 요청한 작업을 처리할 수 있음
    - 완료되면 운영체제는 return-from-trap 특수 명령어를 호출
    - 이 명령어는 특권 수준을 사용자 모드로 다시 하향 조정하면서 호출한 사용자 프로그램으로 리턴
    - 하드웨어는 trap 명령어를 수행할 때 주의가 필요
      - 호출한 프로세스의 필요한 레지스터들을 저장해야 함
        - 운영체제가 return-from-trap 명령어 실행 시 사용자 프로세스로 제대로 리턴할 수 있도록 하기 위함
      - trap이 운영체제 코드의 어디를 실행할지 모르기 때문에 커널은 trap 발생 시 어떤 코드를 실행할지 신중하게 통제해야 함

<br>

- 커널은 부팅 시에 **트랩 테이블(trap table)** 을 만들고 이를 이용하여 시스템을 통제
- 컴퓨터가 부팅될 때는 커널 모드에서 동작하기 때문에 하드웨어를 원하는대로 제어할 수 있음
- 운영체제가 하는 초기 작업 중 하나는 하드웨어에게 예외 사건이 일어났을 때 어떤 코드를 실행해야 하는지 알려줌
- 운영체제는 특정 명령어를 사용하여 하드웨어에게 **트랩 핸들러(trap handler)** 의 위치를 알려줌
  - 하드웨어는 이 정보를 전달받으면 해당 위치를 기억하고 있다가 시스템 콜과 같은 예외적인 사건이 발생했을 때 하드웨어는 무엇을 해야 할지(즉, 어느 코드로 분기하여 실행할지) 알 수 있음
- 제한적 직접 실행 프로토콜
  ```
    운영체제 @부트                            하드웨어
    (커널 모드)
    ----------------------------------------------------------------------------------------------------------
    트랩 테이블을 초기화한다
                                              syscall 핸들러의 주소를 기억한다
  
    운영체제 @실행                            하드웨어                                프로그램
    (커널 모드)                                                                       (사용자 모드)
    ----------------------------------------------------------------------------------------------------------
    프로세스 목록에 항목을 추가한다
    프로그램을 위한 메모리를 할당한다
    프로그램을 메모리에 탑재한다
    argv를 사용자 스택에 저장한다
    레지스터와 PC를 커널 스택에 저장한다
    return-from-trap
                                            커널 스택으로부터 레지스터를 복원한다
                                            사용자 모드로 이동한다
                                            main으로 분기한다
                                                                                      main()을 실행한다
                                                                                      · · ·
                                                                                      시스템 콜을 호출한다
                                                                                      운영체제로 트랩한다
                                            레지스터를 커널 스택에 저장한다
                                            커널 모드로 이동한다
                                            트랩 핸들러로 분기한다
    트랩을 처리한다
    syscall의 임무를 수행한다
    return-from-trap
                                            커널 스택으로부터 레지스터를 복원한다
                                            사용자 모드로 이동한다
                                            트랩 이후의 PC로 분기한다
                                                                                      main에서 리턴한다
                                                                                      trap(exit()를 통하여)
    프로세스의 메모리를 반환한다
    프로세스 목록에서 제거한다
  ```


<br>

#### 1.3. 문제점 2 : 프로세스 간 전환
- 프로세스 간 전환은 간단해야 하며 운영체제는 실행 중인 프로세스를 계속 실행할 것인지, 멈추고 다른 프로세스를 실행할 것인지를 결정해야 함
 
<br>

##### 협조 방식 : 시스템 콜 기다리기
- **협조(cooperative)** 방식으로 알려진 방법은 과거의 몇몇 시스템에서 채택되었던 방식
  - 이 방식에서는 운영체제가 프로세스들이 합리적으로 행동할 것이라고 신뢰함
  - 너무 오랫동안 실행할 가능성이 있는 프로세스는 운영체제가 다른 작업을 실행할 결정을 할 수 있도록 주기적으로 CPU를 포기할 것이라고 가정
  - 대부분의 프로세스는 자주 시스템 콜을 호출하여 CPU의 제어권을 운영체제에게 넘겨줌
  - 이런 유형의 운영체제는 yield 시스템 콜을 제공하는데, 운영체제에게 제어를 넘겨 운영체제가 다른 프로세스를 실행할 수 있게함

<br>

- 응용 프로그램이 비정상적인 행위를 하게 되면 운영체제에게 제어가 넘어감
  - 예를 들어 응용 프로그램이 어떤 수를 0으로 나누는 연산을 실행하거나 접근할 수 없는 메모리에 접근하려고 하면 운영체제로의 트랩이 일어남
  - 그러면 운영체제는 다시 CPU를 획득하여 해당 행위를 하는 프로세스를 종료할 수 있음
- 협조 방식의 스케줄링 시스템에서 운영체제는 시스템 콜이 호출되기를 기다리거나 불법적인 연산이 일어나기를 기다려서 CPU의 제어권을 다시 획득함
 
<br>

##### 비협조 방식 : 운영체제가 전권을 행사
- 프로세스가 시스템 콜을 호출하기를 거부하거나 실수로 호출하지 않아 운영체제에게 제어를 넘기지 않을 경우 하드웨어의 추가적인 도움 없이는 운영체제가 할 수 있는 일은 거의 없음
- 해결책은 **타이머 인터럽트(timer interrupt)** 를 이용하는 것
- 타이머 장치는 수 밀리 초마다 인터럽트를 발생시키도록 프로그램 가능
- 인터럽트가 발생하면 현재 수행 중인 프로세스는 중단되고 미리 구성된 운영체제의 **인터럽트 핸들러(interrupt handler)** 가 실행됨
- 이 시점에 운영체제는 CPU 제어권을 다시 얻게 되고 자신이 원하는 일을 할 수 있음

<br>

- 운영체제는 하드웨어에게 타이머 인터럽트가 발생했을때 실행해야 할 코드를 알려주어야 함
- 부팅 과정 진행 중에 운영체제는 타이머를 시작하며, 타이머가 시작되면 운영체제는 자신에게 제어가 돌아올 것이라는 것을 알고 부담 없이 사용자 프로그램을 실행할 수 있음
- 또한, 타이머는 특정 명령어를 수행하여 끌 수도 있음

<br>

- 인터럽트 발생 시 하드웨어에게도 약간의 역할이 주어지는데 인터럽트가 발생했을 때 실행중이던 프로그램의 상태를 저장하여 나중에 return-from-trap 명령어가 프로그램을 다시 시작할 수 있도록 해야 함
- 이러한 일련의 동작은 시스템 콜이 호출되었을 때 하드웨어가 하는 동작과 매우 유사함
- 다양한 레지스터가 커널 스택에 저장되고, return-from-trap 명령어를 통하여 복원됨

<br>

##### 문맥의 저장과 복원
- 시스템 콜을 통하여 협조적으로 하던 또는 타이머 인터럽트를 통하여 약간은 강제적으로 하던 운영체제가 제어권을 다시 획득하면 현재 실행중인 프로세스를 계속 실행할 것인지 아니면 다른 프로세스로 전환할 것인지를 결정해야 함
- 이 결정은 운영체제의 **스케줄러(scheduler)** 라는 부분에 의해 내려짐
- 다른 프로세스로 전환하기로 결정되면 운영체제는 **문맥 교환(context switch)** 이라고 알려진 코드를 실행
  - 운영체제가 해야하는 작업은 현재 실행 중인 프로세스의 레지스터 값을 커널 스택 같은 곳에 저장하고 곧 실행될 프로세스의 커널 스택으로부터 레지스터 값을 복원하는 것이 전부
  - 그렇게 함으로써 운영체제는 return-from-trap 명령어가 마지막으로 실행될 때 현재 실행중이던 프로세스로 리턴하는 것이 아니라 다른 프로세스로 리턴하여 실행을 다시 시작할 수 있음

<br>

- 프로세스 전환을 위하여 운영체제는 저수준 어셈블리 코드를 사용하여 현재 실행중인 프로세스의 범용 레지스터, PC뿐 아니라 현재 커널 스택 포인터를 저장
- 그리고 곧 실행될 프로세스의 범용 레지스터, PC를 복원하고 커널 스택을 이 프로세스의 커널 스택으로 전환
- 이로써 운영체제는 인터럽트된 프로세스 문맥에서 전환 코드를 호출하고 실행될 프로세스 문맥으로 리턴할 수 있음
- 운영체제가 마지막으로 return-from-trap 명령어를 실행하면 곧 실행될 프로세스가 현재 실행 중인 프로세스가 되며 그래서 문맥 교환이 마무리가 됨

<br>

- xv6 문맥 교환 코드
  ```
    # void switch(struct context **old, struct context *new);
    #
    # Save current register context in old
    # and then load register context from new.
    .globl switch
    switch:
      # Save old registers
      movl 4(%esp) , %eax     # old포인터를 eax에 넣음
      popl 0(%eax)            # old IP를 저장
      movl %esp, 4(%eax)      # 그리고 스택
      movl %ebx, 8(%eax)      # 그리고 다른 레지스터
      movl %ecx, 12(%eax)
      movl %edx, 16(%eax)
      movl %esi, 20(%eax)
      movl %edi, 24(%eax)
      movl %ebp, 28(%eax)
      
      # Load new registers
      movl 4(%esp) , %eax     # new포인터를 eax에 넣음
      movl 28(%eax) , %ebp    # 다른 레지스터를 복원함
      movl 24(%eax) , %edi
      movl 20(%eax) , %esi
      movl 16(%eax) , %edx
      movl 12(%eax) , %ecx
      movl 8(%eax) , %ebx
      movl 4(%eax) , %esp     # 스택은 이 지점에서 전환됨
      pushl 0(%eax)           # 리턴 주소를 지정된 장소에 넣음
      ret                     # 마지막으로 new 문맥으로 리턴함
  ```
  - context 구조체 old와 new는 구 프로세스와 새 프로세스의 프로세스 구조체 안에 위치해 있음


<br>
<br>

### 2. 스케줄링
#### 2.1. 워크로드에 대한 가정
- 일련의 프로세스들이 실행하는 상황을 워크로드(workload)라고 부르기로 하며 워크로드를 결정하는 것은 정책 개발에 매우 중요한 부분
- 워크로드에 관해 더 잘 알수록 그에 맞게 정책을 정교하게 손질할 수 있음


<br>

#### 2.2. 스케줄링 평가 항목
- 스케줄링 정책의 비교를 위해 **스케줄링 평가 항목(scheduling metric)** 을 결정해야 함
- 스케줄링의 경우 다양한 평가 기준이 존재


<br>

#### 2.3. 선입선출
- 가장 기초적인 알고리즘은 **선입선출(First In First Out, FIFO)** 또는 **선도착선처리(First Come First Served, FCFS)** 스케줄링이라고 알려져 있음
- FIFO는 많은 장점을 가지며 단순하고 구현하기 쉬움
- 하지만 짧은 시간 동안 자원을 사용할 프로세스들이 자원을 오랫동안 사용하는 프로세스의 종료를 기다리는 문제점이 있음


<br>

#### 2.4. 최단 작업 우선(Shortest Job First, SJF)
- 이 원칙은 가장 짧은 실행 시간을 가진 작업을 먼저 실행시킴
- 모든 작업이 동시에 도착한다면 SJF가 **최적(optimal)** 의 스케줄링 알고리즘임을 증명할 수 있음


<br>

#### 2.5. 최소 잔여시간 우선
- SJF에 선점 기능을 추가한 **최단 잔여시간 우선(Shortest Time-to-Completion First, STCF)** 또는 **선점형 최단 작업 우선(PSJF)** 으로 알려진 스케줄러
- 언제든 새로운 작업이 시스템에 들어오면 이 스케줄러는 남아 있는 작업과 새로운 작업의 잔여 실행 시간을 계산하고 그 중 가장 적은 잔여 실행 시간을 가진 작업을 스케줄함


<br>

#### 2.6. 새로운 평가 기준 : 응답 시간
- 시분할 컴퓨터가 등장하면서 이제 사용자는 터미널에서 작업하게 되어 시스템에게 상호작용을 원활히 하기위한 성능을 요구하게 됨
<br> -> 응답 시간(response time)이라는 새로운 평가 기준이 생겨남
- 응답 시간은 작업이 도착할 때부터 처음 스케줄 될 때까지의 시간으로 정의


<br>

#### 2.7. 라운드 로빈
- 응답 시간 문제를 해결하기 위하여 **라운드 로빈(Round-Robin, RR)** 스케줄링 알고리즘을 도입
- RR은 작업이 끝날때까지 기다리지 않으며 대신 일정 시간 동안 실행한 후 실행 큐의 다음 작업으로 전환함
- 이때 작업이 실행되는 일정 시간을 **타임 슬라이스(time slice)** 또는 **스케줄링 퀀텀(scheduling quantum)** 이라 부름
- 작업이 완료될 때까지 이런 식으로 계속 진행됨
- 이러한 이유로 RR은 때때로 타임 슬라이싱이라고 불림
- 타임 슬라이스의 길이는 타이머 인터럽트 주기의 배수여야 함
- 타임 슬라이스의 길이는 RR에게 매우 중요하며, 타임 슬라이스가 짧을수록 응답 시간 기준으로 RR의 성능은 더 좋아짐
- 그러나 타임 슬라이스를 너무 짧게 지정하면 문맥 교환 비용이 전체 성능에 큰 영향을 미치게 됨
- 적당한 길이의 응답 시간이 유일한 평가 기준인 경우 타임 슬라이스를 가진 RR은 매우 훌륭한 스케줄러
- 반환 시간이 유일한 측정 기준인 경우 RR은 최악의 정책


<br>

#### 2.8. 입출력 연산의 고려
- 입출력 작업을 요청한 경우 스케줄러는 다음에 어떤 작업을 실행할지를 결정해야 함
- 현재 실행 중인 작업은 입출력이 완료될 때까지 CPU를 사용하지 않을 것이기 때문
- 입출력 요청을 발생시킨 작업은 입출력 완료를 기다리며 대기 상태가 됨
- 입출력이 하드 디스크 드라이브에 보내진 경우 프로세스는 드라이브의 현재 입출력 워크로드에 따라 몇 초 또는 좀 더 긴 시간 동안 블록될 것이며 스케줄러는 그 시간 동안 실행될 다른 작업을 스케줄 해야 함
- 마찬가지로 스케줄러는 입출력 완료 시에도 의사 결정을 해야하며 입출력이 완료되면 인터럽트가 발생하고 운영체제가 실행되어 입출력을 요청한 프로세스를 대기상태에서 준비 상태로 다시 이동시킴
- 물론, 인터럽트가 발생했을 때 요청 프로세스를 즉시 실행시키기로 결정할 수도 있음



<br>
<br>

### 3. 스케줄링_멀티 레벨 피드백 큐(Multi-level Feedback Queue, MLFQ)
- 멀티 레벨 피드백 큐(MLFQ) 스케줄러는 Compatible Time-Sharing System(CTSS)에 사용되며 Corbato 등에 의해 1962년에 최초로 소개됨
- 이 스케줄러는 수년 동안 다듬어져 일부 현대 시스템에까지 발전됨

<br>

- MLFQ가 해결하려고 하는 기본적인 문제 두 가지
  - 첫째, 짧은 작업을 먼저 실행시켜 반환 시간을 최적화하고자 함
    - SJF나 STCF 같은 알고리즘은 작업의 실행 시간 정보를 필요로 하지만, 운영체제는 이 실행 시간을 미리 알 수 없음
  - 둘째, MLFQ는 대화형 사용자에게 응답이 빠른 시스템이라는 느낌을 주고 싶었기 때문에 응답 시간을 최적화함
    - RR과 같은 알고리즘은 응답시간을 단축시키지만 반환 시간은 거의 최악


<br>

#### 3.1. MLFQ : 기본 규칙
- 현재 구현되어 있는 여러 MLFQ들은 자세하게 살펴보면 차이가 있지만, 기본적으로 비슷한 방법을 사용하고 있음
- MLFQ는 여러 개의 큐로 구성되며, 각각 다른 **우선순위(priority level)** 가 배정됨
- 실행 준비가 된 프로세스는 이 중 하나의 큐에 존재함
- MLFQ는 실행할 프로세스를 결정하기 위하여 우선순위를 사용하며 높은 우선순위 큐에 존재하는 작업이 선택됨
- 큐에 둘 이상의 작업이 존재할 수 있으며 이들은 모두 같은 우선순위를 가지고, 이 작업들 사이에서는 라운드 로빈(Round-Robin, RR) 스케줄링 알고리즘이 사용됨

- MLFQ 스케줄링의 핵심은 우선순위를 정하는 방식
- MLFQ는 각 작업에 고정된 우선순위를 부여하는 것이 아니라 각 작업의 특성에 따라 동적으로 우선순위를 부여
- MLFQ는 작업이 진행되는 동안 해당 작업의 정보를 얻고, 이 정보를 이용하여 미래 행동을 예측
- MLFQ의 두 가지 기본 규칙
  - 규칙 1 : Priority(A) > Priority(B) 이면, A가 실행됨(B는 실행되지 않음)
  - 규칙 2 : Priority(A) = Priority(B) 이면, A와 B는 RR 방식으로 실행됨


<br>

#### 3.2. 시도 1: 우선순위의 변경
- 규칙 3 : 작업이 시스템에 진입하면, 가장 높은 우선순위, 즉 맨 위의 큐에 놓여짐
- 규칙 4a : 주어진 타임 슬라이스를 모두 사용하면 우선순위는 낮아짐, 즉 한 단계 아래 큐로 이동함
- 규칙 4b : 타임 슬라이스를 소진하기 전에 CPU를 양도하면 같은 우선순위를 유지함

<br>

##### 현재 MLFQ의 문제점
- 현재 MLFQ는 단순하며 CPU를 긴 작업들과 짧은 작업들 사이에 잘 공유하고 입출력중점 대화형 작업을 빨리 실행시키기 때문에 잘 동작하는 것처럼 보이지만 이 방식은 심각한 결점이 있음
  - 첫째, **기아 상태(starvation)** 가 발생할 수 있음
    - 시스템에 너무 많은 대화형 작업이 존재하면 그들이 모든 CPU 시간을 소모하게 될 것이고 따라서 긴 실행 시간 작업은 CPU 시간을 할당받지 못할 것임
  - 둘째, 똑똑한 사용자라면 스케줄러를 자신에게 유리하게 동작하도록 프로그램을 다시 작성할 수 있음
    - 스케줄러를 자신에게 유리하게 동작시킨다는 것은 일반적으로 스케줄러를 속여서 지정된 몫보다 더 많은 시간을 할당하도록 하게 만드는 것을 가리킴
  - 셋째, 프로그램은 시간 흐름에 따라 특성이 변할 수 있음
    - CPU 위주 작업이 대화형 작업으로 바뀔 수 있음

<br>

- 지금까지 논의한 알고리즘은 아래와 같은 공격에 취약함
  - 타임 슬라이스가 끝나기 전에 아무 파일을 대상으로 입출력 요청을 내려 CPU를 양도하며, 그렇게 하면 같은 큐에 머무를 수 있고 따라서 더 높은 퍼센트의 CPU 시간을 얻게됨



<br>

#### 3.3. 시도 2 : 우선순위의 상향 조정
- CPU 위주 작업이 조금이라도 진행하는 것을 보장하기 위해서 할 수 있는 것
  - 간단한 아이디어는 주기적으로 모든 작업의 우선순위를 상향 조정(boost) 하는 것

<br>

- 규칙 5 : 일정 기간 S가 지나면, 시스템의 모든 작업을 최상위 큐로 이동시킴
  - S값은 너무 크면 긴 실행 시간을 가진 작업은 굶을 수 있으며 너무 작으면 대화형 작업이 적절한 양의 CPU 시간을 사용할 수 없게 됨


<br>

#### 3.4. 시도 3 : 더 나은 시간 측정
- 스케줄러를 자신에게 유리하게 동작시키는 것을 막는 법
  - MLFQ의 각 단계에서 CPU 총 사용 시간을 측정하는 것
    - 스케줄러는 현재 단계에서 프로세스가 소진한 CPU 사용 시간을 저장하며, 프로세스가 타임 슬라이스에 해당하는 시간을 모두 소진하면 다음 우선순위 큐로 강등됨
    - 타임 슬라이스를 한 번에 소진하든 짧게 여러 번 소진하든 상관 없음 

- 규칙 4a와 4b를 합쳐 하나의 규칙으로 재정의
- 규칙 4 : 주어진 단계에서 시간 할당량을 소진하면(CPU를 몇 번 양도하였는지 상관없이), 우선순위는 낮아짐(즉, 아래 단계의 큐로 이동)



<br>

#### 3.5. MLFQ : 요약
- 알고리즘은 멀티 레벨 큐를 가지고 있으며, 지정된 작업의 우선순위를 정하기 위하여 피드백을 사용함
- 과거에 보여준 행동이 우선순위 지정의 지침이 됨
- MLFQ 규칙 전체 정리
- 규칙 1 : 우선순위 (A)> 우선순위 (B)일 경우, A가 실행, B는 실행되지 않음
- 규칙 2 : 우선순위 (A) = 우선순위 (B)일 경우, A와 B는 RR 방식으로 실행됨
- 규칙 3 : 작업이 시스템에 들어가면 최상위 큐에 배치됨
- 규칙 4 : 작업이 지정된 단계에서 배정받은 시간을 소진하면(CPU를 포기한 횟수와 상관없이), 작업의 우선순위는 감소함(즉, 한 단계 아래 큐로 이동)
- 규칙 5 : 일정 주기 S가 지난 후, 시스템의 모든 작업을 최상위 큐로 이동시킴

<br>

- MLFQ는 작업의 특성에 대한 정보 없이 작업의 실행을 관찰한 후 그에 따라 우선순위를 지정함
- MLFQ는 반환 시간과 응답 시간을 모두 최적화함
  - 짧게 실행되는 대화형 작업에 대해서는 우수한 전반적인 성능을 제공하며 오래 실행되는 CPU-집중 워크로드에 대해서는 공정하게 실행하고 조금이라도 진행되도록 함
- 이런 이유로 BSD Unix와 여기서 파생된 다양한 운영체제, Solaris, Windows NT 및 이후 Windows 운영체제를 포함한 많은 시스템이 기본 스케줄러로 MLFQ를 사용

