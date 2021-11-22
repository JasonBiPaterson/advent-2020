(ns advent-2020.day-06
  (:require [clojure.java.io :as io]
            [clojure.string :as string]
            [clojure.set :as set]))

(defn process-input [i]
  (->> (string/split i #"\n\n")
       (map string/split-lines)))

(def input
  (-> "advent_2020/day_06/input"
      io/resource
      slurp
      process-input))

(def part-1 (->> input
                 (map (comp count set string/join))
                 (reduce +)))

(def part-2
  (reduce + (for [groups input]
              (->> (map set groups)
                   (apply set/intersection)
                   count))))
