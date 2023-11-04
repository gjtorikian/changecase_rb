# frozen_string_literal: true

require_relative "changecase_rb/version"
require_relative "changecase_rb/changecase_rb"

class ChangecaseRb # rubocop:disable Style/Documentation
  class Error < StandardError; end
  # Your code goes here...

  def initialize(str)
    @str = str
  end

  def alternate_case
    to_alt_case # calls the underlying Rust function
  end
end
