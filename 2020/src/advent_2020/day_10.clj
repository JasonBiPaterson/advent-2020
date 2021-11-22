(ns advent-2020.day-10
  (:require [clojure.java.io :as io]
            [clojure.math.numeric-tower :as math]
            [clojure.string :as string]))

(def input
  (->> "advent_2020/day_10/input"
       io/resource
       slurp
       string/split-lines
       (map #(Long/parseLong %))
       sort))

(def steps
  (let [coll (concat [0] input [(+ 3 (last input))])]
    (->> (map (fn [a b] (- b a))
              coll
              (rest coll)))))

(def part-1
  (->> steps
       frequencies
       vals
       (reduce *)))

(def one-runs
  (->> (partition-by identity steps)
       (filter (fn [[a]] (= 1 a)))
       (map count)))

;; TODO Understand the combinatorics, this was a guess
(def part-2
  (->> (frequencies one-runs)
       (map (fn [[k v]]
              (math/expt
               (case k
                 1 1
                 2 2
                 3 4
                 4 7)
               v)))
       (reduce *)))
