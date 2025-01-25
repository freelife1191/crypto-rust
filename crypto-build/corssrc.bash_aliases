#!/usr/bin/env bash
# 이 파일에는 환경 변수 정의와 cross를 구성할 수 있는
# 값이 들어 있습니다. 제공된 값은 일반적으로 기본값이며,
# 그렇지 않으면 기본값이 문서화되어 있습니다.

# 교차 구성
# -------------------
# 이는 cross를 구성하는 데 사용되는 환경 변수입니다.

# cross를 실행할 컨테이너 엔진입니다. 기본값은 `docker`, 그 다음 `podman`입니다.
# 먼저 발견된 것이 기본값입니다(예: docker, FAQ 참조). 또한
# 경로나 경로에 있는 바이너리의 이름을 제공할 수 있습니다.
CROSS_CONTAINER_ENGINE=podman

# xargo의 홈 디렉토리입니다.
XARGO_HOME=~/.xargo

# Nix 스토어의 디렉토리입니다.
NIX_STORE=/nix/store

# cross 명령의 사용자 식별자를 설정합니다. 유효한 사용자 ID여야 합니다.
# 유효한 사용자 ID는 일반적으로 `0`(루트) 또는 `1000-60000`(일반 사용자)입니다.
# 유효한 사용자 ID에 대한 사양은 여기에 설명되어 있습니다.
# https://en.wikipedia.org/wiki/User_identifier#Conventions
CROSS_CONTAINER_UID=1000

# cross 명령에 대한 그룹 식별자를 설정합니다. 여기에는
# 사용자 ID와 유사한 제한이 있습니다.
CROSS_CONTAINER_GID=1000

# cross에 컨테이너 내부에서 실행 중임을 알립니다. 설정하지 않으면
# `0`, 비어 있음 또는 `false`로 설정하면 `cross`에 컨테이너에서 실행 중이 아님을 알립니다.
# 기본적으로 이것은 false입니다.
CROSS_CONTAINER_IN_CONTAINER=1

# $engine 실행 중 컨테이너 엔진에 제공할 추가 인수입니다.
# `cross`에 전달된 추가 사용자 지정 또는 플래그가 필요할 때 유용합니다.
# 다른 구성 변수에서 사용할 수 없습니다.
CROSS_CONTAINER_OPTS="--env MYVAR=1"

# `cross` 구성 파일의 경로를 지정합니다.
CROSS_CONFIG="Cross.toml"

# Linux 또는 Android 러너를 사용할 때 cross에 대한 디버깅 정보를 출력합니다.
# 설정하지 않으면 `0`, 비어 있음 또는 `false`로 설정하면 디버깅 정보를 출력하지 않습니다.
# 기본적으로 false입니다.
CROSS_DEBUG=1

# 이전 cross 동작을 사용합니다. 기본적으로 설정하지 않고 최신 동작을 사용합니다.
CROSS_COMPATIBILITY_VERSION=0.2.1

# rustup이 사용자 지정 툴체인을 사용 중이므로
# 대상을 추가하거나 구성 요소를 설치하지 않아야 함을 지정합니다. `cargo-bisect-rustc`와 함께 사용하면 유용합니다.
# 설정하지 않으면 `0`, 비어 있음 또는 `false`로 설정하면 rustup이 대상 또는 구성 요소를 설치할 수 있다고 가정합니다. 기본적으로 false입니다.
CROSS_CUSTOM_TOOLCHAIN=1

# 원격 컨테이너 엔진을 사용하고 로컬 바인드 마운트가 아닌 데이터 볼륨을 사용한다는 것을 크로스에 알립니다.
# 원격 컨테이너 엔진을 사용하는 방법에 대한 자세한 내용은 원격을 참조하세요. 설정 해제 시 `0`, 비어 있음 또는 `false`로 설정하면,
# 크로스가 로컬에서 실행 중이라고 가정합니다. 기본적으로 이는 false입니다.
CROSS_REMOTE=1

# Qemu를 사용하여 실행한 바이너리에서 시스템 호출의 백트레이스를 가져옵니다.
# 기본적으로 "외부"(비 x86_64)만 실행할 때 Qemu를 사용합니다. 설정 해제 시
# `0`, 비어 있음 또는 `false`로 설정하면 strace를 제공하지 않습니다. 기본적으로 이는 false입니다.
QEMU_STRACE=1

# 컨테이너 엔진이 루트로 실행되거나 루트가 없는지 여부를 지정합니다.
# 설정 해제 시 `auto`, `0`, empty 또는 `false`로 설정하면 cross는 docker가
# root로 실행되고 다른 모든 컨테이너 엔진은 root가 없다고 가정합니다.
CROSS_ROOTLESS_CONTAINER_ENGINE=1

# cargo 레지스트리와 git 디렉토리를 복사합니다. 개인 SSH 종속성을 지원하려면
# 활성화해야 합니다. 설정 해제 시 `auto`,
# `0`, empty 또는 `false`로 설정하면 cross는 레지스트리를 복사하지 않으므로
# 원격 데이터 볼륨에서 다시 만들어야 합니다.
CROSS_REMOTE_COPY_REGISTRY=1

# CACHETAG.DIR(캐시 디렉토리 태그)을 포함한 모든 디렉토리를 복사합니다. 설정 해제 시 `auto`, `0`, empty 또는 `false`로 설정하면
# cross는 모든 캐시 디렉토리를 건너뛰어 원격 클라이언트로 전송되는 데이터 양을 줄입니다.
# CROSS_REMOTE_COPY_CACHE=1

# 빌드를 완료한 후 생성된 빌드 아티팩트를 호스트로 다시 복사하지 마십시오. 영구 데이터 볼륨을 사용하는 경우 아티팩트는
# 볼륨에 남아 있습니다. 로컬에서 변경 사항을 테스트하는 동안 원격으로 빌드하는 경우 유용합니다. 즉, 생성된 빌드 아티팩트가
# 항상 또는 전혀 필요하지 않은 경우입니다. 설정하지 않거나 `auto`, `0`, empty,
# 또는 `false`로 설정하면 대상 디렉토리를 호스트로 다시 교차 복사합니다.
CROSS_REMOTE_SKIP_BUILD_ARTIFACTS=1

# 컨테이너 사용자 네임스페이스를 사용자 지정합니다. none으로 설정하면 사용자 네임스페이스가
# 비활성화됩니다. 제공되지 않거나 `auto`로 설정하면
# 기본 네임스페이스를 사용합니다.
CROSS_CONTAINER_USER_NAMESPACE="host"

# CROSS TOML 구성
# ------------------------
# 이는 `Cross.toml` 또는 `Cargo.toml`에 있는 구성 값을 지정하는 추가 방법입니다. 환경 변수는 `Cross.toml`에서 찾은
# 값을 재정의하지만, 덜 구체적인 값보다 더 구체적인 값이
# 여전히 사용됩니다. 예를 들어, `target.(...).xargo`
#는 여전히 `CROSS_BUILD_XARGO`를 재정의합니다. 우선순위는 다음과 같습니다.
# 1. 환경 변수 대상.
# 2. 구성 대상.
# 3. 환경 변수 빌드.
# 4. 구성 빌드.
#
# `Cross.toml`의 값은 `Cargo.toml`의 값도 재정의합니다. 거의
# 모든 구성 옵션을 제공할 수 있으며, `build.xargo`는
# `CROSS_BUILD_XARGO`로, `build.default-target`는
# `CROSS_BUILD_DEFAULT_TARGET`로, `target.aarch64-unknown-linux-gnu.runner`는
# `C로 제공됩니다.