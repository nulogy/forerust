#!/usr/bin/env ruby

puts "Hello from STDOUT test 1!"
puts "Environment variable FOO: #{ENV['FOO']}"
STDERR.puts "Hello from STDERR!"
