# rhwp 멀티플랫폼 확장 계획서 (Desktop & Android)

이 문서는 `rhwp` 엔진을 기반으로 한 데스크톱 앱 및 안드로이드 앱("뉴아래한글") 개발을 위한 상세 계획을 담고 있다 이기.

## 1. 목표
- **데스크톱 앱**: 기존 `rhwp-studio`를 Tauri 기반의 네이티브 앱으로 래핑하여 설치형 성능 및 로컬 파일 접근성 확보.
- **안드로이드 앱 ("뉴아래한글")**: 한컴오피스 뷰어를 대체할 수 있는 고성능 뷰어/에디터 앱 개발 (Kotlin + Gradle + Rust Engine).

---

## 2. 기술 스택 및 아키텍처

### 2.1 데스크톱 (rhwp-desktop)
- **Framework**: Tauri v2 (Rust Backend + Vite/TS Frontend)
- **Frontend**: 기존 `rhwp-studio` 코드 재사용 및 Tauri API 연결
- **Backend**: `src/`의 Rust 코드를 직접 호출 (WASM 오버헤드 제거)

### 2.2 안드로이드 (뉴아래한글 / NewAraHangul)
- **Language**: Kotlin
- **Build System**: Gradle (Kotlin DSL)
- **Rust Integration**: `UniFFI` 또는 `JNI` + `cargo-ndk`
- **Rendering**: `rhwp` 엔진의 SVG/Canvas 렌더링 결과를 Android `Canvas` 또는 `WebView`에 바인딩
- **UI**: Jetpack Compose (모던 UI 설계)

---

## 3. 상세 단계별 계획

### Phase 1: 데스크톱 앱 기초 (Setup Desktop)
1. **Tauri 초기화**: 프로젝트 루트에 Tauri 툴체인 설정.
2. **Frontend 통합**: `rhwp-studio` 빌드 결과물을 Tauri와 연동.
3. **네이티브 빌드**: `cargo tauri build` 확인 (Native Windowing).

### Phase 2: 안드로이드 앱 기초 (Setup Android)
1. **Gradle 프로젝트 생성**: `rhwp-android` 디렉토리에 Kotlin/Compose 기반 프로젝트 생성.
2. **Rust Library 패키징**: `cdylib` 형태로 Rust 코드를 컴파일하여 `.so` 파일 생성.
3. **Kotlin-Rust 브릿지**: `UniFFI`를 활용하여 Rust의 `HwpDocument` 객체를 Kotlin에서 직접 사용 가능하게 연동.

### Phase 3: "뉴아래한글" 기능 구현
1. **파일 탐색기**: 디바이스 내 `.hwp`, `.hwpx` 파일 검색 및 로드 기능.
2. **렌더링 뷰**: Rust 엔진에서 생성한 SVG/Bitmap을 안드로이드 화면에 렌더링.
3. **오프라인 폰트 지원**: 안드로이드 시스템 폰트 및 사용자 정의 폰트 연동.

---

## 4. 브랜치 전략 및 워크플로우

> [!IMPORTANT]
> 모든 머지 및 PR은 사용자(`rheehose`)의 명시적 승인이 있을 때만 수행한다.

### 브랜치 구조
- `main`: 안정화 릴리즈용
- `devel`: 개발 통합 브랜치
- `feature/desktop`: 데스크톱 앱 개발 전용
- `feature/android`: 안드로이드 앱 개발 전용
- `local/task{N}`: 개별 기능 개발용 (Issue 기반)

### 작업 규칙
1. `gh issue create`를 통해 타스크 번호 할당.
2. 각 플랫폼별 `feature/` 브랜치에서 분기하여 작업.
3. 작업 완료 후 `devel` 브랜치로의 PR 생성 (머지는 사용자 명령 대기).

---

## 5. 사용자 확인 사항
- 데스크톱 앱을 새로 만들 때 Tauri를 사용하는 것에 동의하는지?
- 안드로이드 앱의 패키지 성능을 위해 `UniFFI`를 사용하는 방식에 동의하는지?
- 안드로이드 앱의 뷰어 기능 외에 기초적인 편집 기능까지 포함할지?

---

## 6. 검증 계획
- **데스크톱**: `cargo tauri dev`를 통해 로컬 파일 로딩 속도 및 UI 반응성 검증.
- **안드로이드**: 에뮬레이터 및 실기기에서 `gradle build` 후 APK 설치 및 HWP 렌더링 품질 검증.
