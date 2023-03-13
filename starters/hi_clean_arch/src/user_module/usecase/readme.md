
https://fullstackmark.com/post/11/better-software-design-with-clean-architecture

## use case description
Title	Register for courses

Description	Student accesses the system and views the courses currently available for him to register. Then he selects the courses and registers for them.

Primary Actor	Student

Preconditions	
Student is logged into system
Student has not previously enrolled or registered
Student cannot register within 5 days of course start date
Postconditions	Student is registered for courses
Main Success Scenario	
Student selects "Register New Courses" from the menu.
System displays list of courses available for registering.
Student selects one or more courses he wants to register for.
Student clicks "Submit" button.
System registers student for the selected courses and displays a confirmation message.
Extensions	
(2a) No courses are available for this student.
System displays error message saying no courses are available, and provides the reason & how to rectify if possible.
Student either backs out of this use case, or tries again after rectifying the cause.
(5a) Some courses could not be registered.
System displays message showing which courses were registered, and which courses were not registered along with a reason for each failure.
(5b) None of the courses could be registered.
System displays message saying none of the courses could be registered, along with a reason for each failure.