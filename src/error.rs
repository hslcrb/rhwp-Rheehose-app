//! 도메인 에러 타입
//!
//! 파서, 렌더러, 커맨드 등 크레이트 전역에서 사용하는 에러 열거형.

/// 네이티브 에러 타입 (non-WASM 환경에서도 안전하게 사용)
#[derive(Debug, uniffi::Error)]
pub enum HwpError {
    /// 파일이 유효하지 않음
    InvalidFile { message: String },
    /// 페이지 범위 초과
    PageOutOfRange { page: u32 },
    /// 렌더링 오류
    RenderError { message: String },
    /// 필드 관련 오류
    InvalidField { message: String },
}

impl From<crate::parser::ParseError> for HwpError {
    fn from(e: crate::parser::ParseError) -> Self {
        HwpError::InvalidFile { message: format!("{:?}", e) }
    }
}

impl From<crate::parser::hwpx::HwpxError> for HwpError {
    fn from(e: crate::parser::hwpx::HwpxError) -> Self {
        HwpError::InvalidFile { message: format!("{:?}", e) }
    }
}

impl From<crate::serializer::SerializeError> for HwpError {
    fn from(e: crate::serializer::SerializeError) -> Self {
        HwpError::RenderError { message: format!("{:?}", e) }
    }
}

impl std::fmt::Display for HwpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HwpError::InvalidFile { message } => write!(f, "유효하지 않은 파일: {}", message),
            HwpError::PageOutOfRange { page } => write!(f, "페이지 {}을(를) 찾을 수 없습니다", page),
            HwpError::RenderError { message } => write!(f, "렌더링 오류: {}", message),
            HwpError::InvalidField { message } => write!(f, "필드 오류: {}", message),
        }
    }
}
