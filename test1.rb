#!/usr/bin/env ruby

$stdout.sync = true

STDERR.puts "Hello from STDERR!"
puts "Environment variable FOO: #{ENV['FOO']}"
puts "Hello from STDOUT test 1!"
sleep 2
puts "Hello test 1 again"
