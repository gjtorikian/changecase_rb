# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/test_*.rb"]
end

require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("changecase_rb.gemspec")

RbSys::ExtensionTask.new("changecase_rb", GEMSPEC) do |ext|
  ext.lib_dir = "lib/changecase_rb"
end

task default: %i[compile test rubocop]
