#!/usr/bin/env ruby

$stdout.sync = true

puts "Hello from STDOUT test 1!"
sleep 2
puts "Hello test 1 again"
STDERR.puts "Hello from STDERR!"
