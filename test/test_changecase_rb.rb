# frozen_string_literal: true

require "test_helper"

class TestChangecaseRb < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::ChangecaseRb::VERSION
  end

  def test_it_does_something_useful
    assert_equal "HeLlO wOrLd", ChangecaseRb.new("Hello world").alternate_case
  end
end
