# working
Line 3	// It tells rust to import a specific smart pointer

Line 5	// tells rust to duplicate switch without complaining

Line 7	// 2 way switch between Pending and Completed

Line 9	// Simple wrapper around a list of string logs

Line 10	// Holds a list of log messages

Line 12	// starts def of task

Line 13	// tasks title as text "Build API"

Line 14	// the switch of pend or comp so it can be changed later

Line 15	// safe link to parents task

Line 16	// lets multiple parents share the same task

Line 17	// shared key to access central log book

Line 20	// defining functions for task

Line 21	// takes shared name and logger and wraps new task in rc

Line 22	// builds task and puts in shared rc key

Line 23	// makes a permanent owned copy of the name string

Line 24	// sets pend status in refcell so it can be switched later

Line 25	// starts with no parent so it can be attached to one later

Line 26	// starts empty subtask list so subtask can be attached later

Line 28	// stores shared logger key

Line 33	// takes parent and child task and link them tgth

Line 34	// open parents subtask key and adds strong key(rc)

Line 35	// demote parents rc in weak and give to child

Line 37	// marks a finished task and upd logs

Line 38	// open task status and flip to completed

Line 39	// open share logbook so we can write inside

Line 40	// writes take name completed in logbook

Line 43	// Safely upgrade the Weak pointer and log if parent still exists

Line 44	// checks if parent exist and pgrd to active key

Line 45	// if parent still alive write notified(parents name)

Line 49	// main entry point where program start running

Line 50	// creates empty logbook in refcell so it can be edited

Line 52	// Creates the "Build API" parent task and hands it a key to the shared logger.

Line 53	// Creates the "Setup DB" child task and hands it a key to the shared logger.

Line 55	// Links them together (Parent gets key to Child; Child gets business card to Parent).

Line 56	// Marks "Setup DB" as completed, writes to the logbook, and notifies "Build API"
Line 58	// Print all logged messages
