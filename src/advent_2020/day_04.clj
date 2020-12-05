(ns advent-2020.day-04
  (:require [clojure.java.io :as io]
            [clojure.string :as string]))

(def input
  (-> "advent_2020/day_04/input"
      io/resource
      slurp
      string/trim
      (string/split #"\n\n")))

(def passports
  (for [line input]
    (->> (string/split line #"\s")
         (map (fn [s] (update (string/split s #":") 0 keyword)))
         (into {}))))

(def required-keys [:byr :iyr :eyr :hgt :hcl :ecl :pid])

(def passports-with-keys
  (for [passport passports
         :when (every? some?
                       ((apply juxt required-keys)
                        passport))]
     passport))

(def part-1 (count passports-with-keys))

(def part-2
  (count
   (for [passport passports-with-keys
         :let [{:keys [byr iyr eyr hgt hcl ecl pid]} passport]
         :when (and (<= 1920 (Long/parseLong byr) 2002)
                    (<= 2010 (Long/parseLong iyr) 2020)
                    (<= 2020 (Long/parseLong eyr) 2030)
                    (or (and (re-find #"^\d+cm$" hgt)
                             (<= 150
                                 (Long/parseLong (re-find #"^\d+" hgt))
                                 193))
                        (and (re-find #"^\d+in$" hgt)
                             (<= 59
                                 (Long/parseLong (re-find #"^\d+" hgt))
                                 76)))
                    (re-find #"^#[\da-f]{6}$" hcl)
                    (#{"amb" "blu" "brn" "gry" "grn" "hzl" "oth"} ecl)
                    (re-find #"^\d{9}$" pid))]
     passport)))
