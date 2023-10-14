use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
  #[msg("Exceeding maximum number of allowed characters")]
  TooLong,
  #[msg("Your content contains unallowed characters")]
  UnallowedChars,
  #[msg("Please provide content to post")]
  NoContent,
  #[msg("No update detected")]
  NothingChanged,
  #[msg("Alias already exist")]
  AliasPresent,
}